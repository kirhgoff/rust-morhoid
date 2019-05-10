use morphoid::types::*;

// ---------------------------------

// TODO: delete me
impl KillAction {
    pub fn new(x: Coords, y: Coords) -> KillAction {
        KillAction { x, y }
    }
}

impl Action for KillAction {
    fn execute(&self, affector: &mut Affector) {
        affector.set_entity(self.x, self.y, Entity::Corpse(10), None, None);
    }
}

// ---------------------------------

impl UpdateHealthAction {
    pub fn new(x: Coords, y: Coords, health_delta: HealthType) -> UpdateHealthAction {
        UpdateHealthAction { x, y, health_delta }
    }
}

impl Action for UpdateHealthAction {
    fn execute(&self, affector: &mut Affector) {
        //println!("UpdateHealthAction.execute x={:?} y={:?} health_delta={:?}", self.x, self.y, self.health_delta);
        affector.update_health(self.x, self.y, self.health_delta);
    }
}

// --------------------------------

impl ReproduceAction {
    pub fn new(x: Coords, y: Coords) -> ReproduceAction {
        ReproduceAction { x, y }
    }
}

impl Action for ReproduceAction {
    fn execute(&self, affector: &mut Affector) {
        affector.punish_for_action(self.x, self.y, REPRODUCE);
        affector.reproduce(self.x, self.y);
    }
}

// --------------------------------

impl AttackAction {
    pub fn new(x: Coords, y: Coords, damage: HealthType) -> AttackAction {
        AttackAction { x, y, damage }
    }
}

impl Action for AttackAction {
    fn execute(&self, affector: &mut Affector) {
        affector.punish_for_action(self.x, self.y, ATTACK);
        affector.attack(self.x, self.y, self.damage);
    }
}

// --------------------------------

impl MoveAction {
    pub fn new(x: Coords, y: Coords) -> MoveAction {
        MoveAction { x, y }
    }
}

impl Action for MoveAction {
    fn execute(&self, affector: &mut Affector) {
        affector.punish_for_action(self.x, self.y, MOVE);
        affector.move_cell(self.x, self.y);
    }
}

// --------------------------------

impl RotateAction {
    // TODO: find macros that allows to create objects with ()
    pub fn new(x: Coords, y: Coords, value: Gene) -> RotateAction {
        RotateAction { x, y, value }
    }
}

impl Action for RotateAction {
    fn execute(&self, affector: &mut Affector) {
        affector.punish_for_action(self.x, self.y, TURN);
        affector.rotate_cell(self.x, self.y, self.value);
    }
}

// --------------------------------

impl DefileAction {
    pub fn new(x: Coords, y: Coords, damage: HealthType) -> DefileAction {
        DefileAction { x, y, damage }
    }
}

impl Action for DefileAction {
    fn execute(&self, affector: &mut Affector) {
        affector.punish_for_action(self.x, self.y, DEFILE);
        affector.defile(self.x, self.y, self.damage);
    }
}

// --------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_health() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.id();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState::default()));

        assert_eq!(world.get_state(hash).health, 10);

        Processor::new().apply(
            &vec![Box::new(UpdateHealthAction::new(0, 0, 5))],
            &mut world
        );

        assert_eq!(world.get_state(hash).health, 15);
    }

    #[test]
    fn test_reproduce() {
        let mut world = World::prod(2, 1);
        let plant = Genome::new_plant();
        let genome_id = plant.id();
        world.set_cell_ext(0, 0, plant, Direction::East);

        Processor::new().apply(
            &vec![Box::new(ReproduceAction::new(0, 0))],
            &mut world
        );

        match world.get_entity(1, 0) {
            Entity::Cell(new_hash) => {
                assert_ne!(*new_hash, genome_id);
                assert_eq!(world.get_state(*new_hash).health, 10);
            },
            _ => panic!("Cant find reproduced entity")
        }
    }

    #[test]
    fn test_attack() {
        // TODO: this is not needed
        let settings = SettingsBuilder::zero(); // initial health is 10
        let new_value =
            settings.initial_cell_health() +
            settings.attack_cost();

        let mut world = World::new(2, 1, settings);
        world.set_cell_ext(0, 0, Genome::new_plant(), Direction::East);
        world.set_cell_ext(1, 0, Genome::new_predator(), Direction::West);

        Processor::new().apply(
            &vec![Box::new(AttackAction::new(1, 1, 100))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Corpse(_) => {},
            _ => panic!("Cell survived after 100 of damage!"),
        }

        match world.get_entity(1, 0) {
            Entity::Cell(genome_id) => {
                assert!(world.get_state(*genome_id).health > new_value);
            },
            _ => panic!("Predator cell should have high health!"),
        }
    }

    #[test]
    fn test_move() {
        let mut world = World::prod(2, 1);
        let mut plant = Genome::new_plant();
        plant.mutate(0, MOVE);

        let genome_id = plant.id();

        world.set_cell_ext(0, 0, plant, Direction::East);

        Processor::new().apply(
            &vec![Box::new(MoveAction::new(0, 0))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Nothing => { },
            _ => panic!("Cell should have moved away")
        }

        match world.get_entity(1, 0) {
            Entity::Cell(new_hash) => {
                assert_eq!(*new_hash, genome_id);
            },
            _ => panic!("Cell should have moved in")
        }
    }

    #[test]
    fn test_rotate() {
        let mut world = World::prod(1, 1);
        let mut plant = Genome::new_plant();
        plant.mutate(0, TURN);
        plant.mutate(1, 1); // Rotate clockwise by 1

        let genome_id = plant.id();

        world.set_cell_ext(0, 0, plant, Direction::North);

        Processor::new().apply(
            &vec![Box::new(RotateAction::new(0, 0, 1))],
            &mut world
        );

        match world.get_entity(1, 0) {
            Entity::Cell(new_genome_id) => {
                let cell_state = world.get_state(*new_genome_id);
                assert_eq!(Direction::NorthEast, cell_state.direction);
                assert_eq!(genome_id, *new_genome_id);
            },
            _ => {}
        }
    }

    #[test]
    fn test_defile() {
        let settings = SettingsBuilder::prod()
            .with_initial_cell_health(10)
            .with_corpse_initial(10)
            .with_defile_damage(5)
            .with_defile_cost(-3)
            .build();

        let new_health =
            settings.initial_cell_health() +
                settings.defile_cost() +
                settings.defile_damage();

        let mut world = World::new(2, 1, settings);
        world.set_cell_ext(0, 0, Genome::new_predator(), Direction::East);
        world.set_corpse(1, 0, 10);

        Processor::new().apply(
            &vec![Box::new(DefileAction::new(0, 0, 5))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Cell(genome_id) => {
                assert_eq!(new_health, world.get_state(*genome_id).health);
            },
            _ => panic!("There should be a predator"),
        }

        match world.get_entity(1, 0) {
            Entity::Corpse(remains) => {
                assert_eq!(5, *remains)
            },
            _ => panic!("Corpse should stay here"),
        }

    }

}