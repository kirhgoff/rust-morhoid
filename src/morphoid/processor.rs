use morphoid::entity::Entity;
use morphoid::action::*;
use morphoid::world::*;
use morphoid::genome::*;

pub struct Processor {}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    pub fn process_entity(&self, x:Coords, y:Coords, entity: Entity, perceptor: &Perceptor) -> Vec<Box<dyn Action>> {
        let mut all_actions:Vec<Box<dyn Action>> = Vec::new();
        match entity {
            Entity::Cell(genome_id) => {
                let mut actions = self.execute(x, y, genome_id, perceptor);
                all_actions.append(&mut actions);
            }
            _ => {},
        };
        all_actions
    }

    // TODO: move to world
    pub fn apply(&self, actions: &Vec<Box<dyn Action>>, affector: &mut Affector) {
        for action in actions.iter() {
            action.execute(affector);
        }
    }

    pub fn execute(&self, x:Coords, y:Coords, genome_id: HashType, perceptor: &Perceptor) -> Vec<Box<dyn Action>> {
        let mut actions:Vec<Box<dyn Action>> = Vec::new();

        let genome = perceptor.get_genome(genome_id);

        let start_index = 0;
        let steps_limit = 15; // TODO: add settings object - unhardcode
        let end_index = (start_index + steps_limit) % GENE_LENGTH;

        for i in start_index..end_index {
            let gene = genome.genes[i];
            match gene {
                REPRODUCE => {
                    // TODO: unhardcode
                    actions.push(Box::new(UpdateHealthAction::new(x, y, -10)));
                    match perceptor.find_vacant_place_around(x, y) {
                        Some((new_x, new_y)) => actions.push(Box::new(ReproduceAction::new(new_x, new_y, genome_id))),
                        _ => {}
                    }
                }, // 30
                PHOTOSYNTHESYS => actions.push(Box::new(UpdateHealthAction::new(x, y, 5))), // 31
                _ => {}
            }

        }
        actions
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use morphoid::cell_state::CellState;

    #[test]
    fn processor_can_do_kill_entity_action() {
        let mut world = World::new(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState{health: 10}));

        match world.get_entity(0, 0) {
            Entity::Cell(old_hash) => assert_eq!(*old_hash, hash),
            _ => panic!()
        }

        Processor::new().apply(
            &vec![Box::new(KillAction::new(0, 0))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Corpse(_) =>  {},
            _ => panic!()
        }
    }

    #[test]
    fn processor_can_do_update_health() {
        let mut world = World::new(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState { health: 10 }));

        Processor::new().apply(
            &vec![Box::new(UpdateHealthAction::new(0, 0, -100))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Corpse(_) =>  {},
            _ => panic!("Cell should be dead here")
        }
    }
}
