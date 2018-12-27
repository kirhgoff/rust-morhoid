use morphoid::entity::Entity;
use morphoid::action::*;
use morphoid::world::*;
use morphoid::genome::*;
use morphoid::action::KillAction;
use std::collections::LinkedList;
use morphoid::cell_state::CellState;

pub struct Processor {}

impl Processor {
    pub fn new() -> Processor {
        Processor {}
    }

    pub fn process_entity(&self, entity: Entity, perceptor: &mut Perceptor) -> Vec<Box<dyn Action>> {
        let mut all_actions:Vec<Box<dyn Action>> = Vec::new();
        match entity {
            Entity::Cell(genome_id) => {
                let genome = perceptor.get_genome(genome_id);
                let state = perceptor.get_state(genome_id);
                let mut actions = self.execute(genome, state);
                all_actions.append(&mut actions);
            }
            _ => {},
        };
        all_actions
    }

    pub fn apply(&self, actions: &Vec<Box<dyn Action>>, affector: &mut Affector) {
        for action in actions.iter() {
            action.execute(affector);
        }
    }

    pub fn execute(&self, genome: &Genome, cell_state: &CellState) -> Vec<Box<dyn Action>> {
        //TODO
        vec![]
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

        let kill_action = KillAction::new(0, 0);
        let mut list: Vec<Box<Action>> = Vec::new();
        list.push(Box::new(kill_action));

        Processor::new().apply(&list, &mut world);

        let new_entity = world.get_entity(0, 0);
        match new_entity {
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

        let update_health_action = UpdateHealthAction  {x:0, y:0, health_delta: -100};
        let mut list: Vec<Box<Action>> = Vec::new();
        list.push(Box::new(update_health_action));

        Processor::new().apply(&list, &mut world);

        let new_entity = world.get_entity(0, 0);
        match new_entity {
            Entity::Corpse(_) =>  {},
            _ => panic!(format!("{:?}", new_entity))
        }
    }
}
