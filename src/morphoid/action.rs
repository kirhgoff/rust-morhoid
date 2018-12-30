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

pub struct ReproduceAction {
    pub x: Coords,
    pub y: Coords,
    pub parent_genome_id: GenomeId
}

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

}