use morphoid::types::*;

// ---------------------------------

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
    pub fn new(x: Coords, y: Coords, parent_genome_id: GenomeId) -> ReproduceAction {
        ReproduceAction { x, y, parent_genome_id }
    }
}

impl Action for ReproduceAction {
    fn execute(&self, affector: &mut Affector) {
        //println!("ReproduceAction.execute x={:?} y={:?}", self.x, self.y);
        let new_genome = affector.build_child_genome_for(self.parent_genome_id);
        affector.set_entity(self.x, self.y, Entity::Cell(new_genome.hash()), Some(new_genome), Some(CellState {health: 10}));
        // TODO: use settings (initial state health)
    }
}

// --------------------------------

impl AttackAction {
    pub fn new(victim_x: Coords, victim_y: Coords, attacker_x: Coords, attacker_y: Coords, damage: HealthType) -> AttackAction {
        AttackAction { victim_x, victim_y, attacker_x, attacker_y, damage }
    }
}

impl Action for AttackAction {
    fn execute(&self, affector: &mut Affector) {
        affector.update_health(self.victim_x, self.victim_y, -1 * self.damage);
        affector.update_health(self.attacker_x, self.attacker_y, self.damage);
        // TODO: some punishment for not having enough energy?
    }
}
// --------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_health_action_works() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState {health: 10}));

        assert_eq!(world.get_state(hash).health, 10);

        Processor::new().apply(
            &vec![Box::new(UpdateHealthAction::new(0, 0, 5))],
            &mut world
        );

        assert_eq!(world.get_state(hash).health, 15);
    }

    #[test]
    fn reproduce_action_works() {
        let mut world = World::prod(2, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState {health: 10}));

        Processor::new().apply(
            &vec![Box::new(ReproduceAction::new(1, 0, hash))],
            &mut world
        );

        match world.get_entity(1, 0) {
            Entity::Cell(new_hash) => {
                assert_ne!(*new_hash, hash);
                assert_eq!(world.get_state(*new_hash).health, 10);
            },
            _ => panic!("Cant find reproduced entity")
        }
    }

    #[test]
    fn attack_action_works() {
        // TODO: this is not needed
        let settings = Settings {
            steps_per_turn: 2,
            reproduce_cost: -8, // it will die after new born
            reproduce_threshold: 9, // it will reproduce on second step
            photosynthesys_adds: 5, // it will have 10 + 5 health after first step
            initial_cell_health: 10, // it will have 10 originally
            attack_damage: 4,
        };

        let mut world = World::new(2, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());
        world.set_cell(1, 0, Genome::new_predator());

        Processor::new().apply(
            &vec![Box::new(AttackAction::new(0, 0, 1, 1, 100))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Corpse(_) => {},
            _ => panic!("Cell survived after 100 of damage!"),
        }

        match world.get_entity(1, 0) {
            Entity::Cell(genome_id) => {
                assert_eq!(world.get_state(*genome_id).health, 110);
            },
            _ => panic!("Predator cell should have high health!"),
        }
    }

}