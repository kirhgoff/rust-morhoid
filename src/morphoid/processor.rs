use morphoid::entity::Entity;
use morphoid::action::*;
use morphoid::world::*;
use morphoid::genome::*;
use morphoid::action::KillAction;
use std::collections::LinkedList;
use morphoid::cell_state::CellState;

pub struct Processor {}

impl Processor {
    pub fn process_entity<T: Action>(entity: Entity, perceptor: &mut Perceptor) -> Vec<Box<T>> {
        let mut all_actions:Vec<Box<T>> = Vec::new();
        let new_entity = match entity {
            Entity::Cell(genome_id) => {
                let genome = perceptor.get_genome(genome_id);
                let state = perceptor.get_state(genome_id);
                let mut actions = Processor::execute(genome, state);
                all_actions.append(&mut actions);
            }
            otherwise => {},
        };
        all_actions
    }

    pub fn apply<T : Action>(actions: &LinkedList<Box<T>>, affector: &mut Affector) {
        for action in actions.iter() {
            action.execute(affector);
        }
    }

    pub fn execute<T: Action>(genome: &Genome, cell_state: &CellState) -> Vec<Box<T>> {
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
        world.set_entity(0, 0, Entity::Cell(hash), Some(CellState{health: 10}));
        match world.get_entity(0, 0) {
            Entity::Cell(old_hash) => assert_eq!(*old_hash, hash),
            _ => panic!()
        }

        let kill_action = KillAction::new(0, 0);
        let mut list = LinkedList::new();
        list.push_back(Box::new(kill_action));
        Processor::apply(&list, &mut world);

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
        world.set_entity(0, 0, Entity::Cell(hash), Some(CellState { health: 10 }));

        let update_health_action = UpdateHealthAction  {x:0, y:0, health_delta: -100};
        let mut list = LinkedList::new();
        list.push_back(Box::new(update_health_action));
        Processor::apply(&list, &mut world);

        let new_entity = world.get_entity(0, 0);
        match new_entity {
            Entity::Corpse(_) =>  {},
            _ => panic!(format!("{:?}", new_entity))
        }
    }
}
