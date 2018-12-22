use morphoid::entity::Entity;
use morphoid::action::*;
use morphoid::world::*;
use morphoid::genome::*;
use morphoid::action::KillAction;
use std::collections::LinkedList;

pub struct Processor {}

impl Processor {
    pub fn new_entity<T : Action>(entity: Entity, perceptor: &Perceptor) -> (Entity, Vec<T>) {
        //let mut actions:Vec<Action> = Vec::new(); TODO: what to do?
        let new_entity = match entity {
            Entity::Cell(gene_id) => Entity::Cell(gene_id + 1),
            otherwise => otherwise,
        };
        (new_entity, vec![])
    }

    pub fn apply<T : Action>(actions: &LinkedList<T>, affector: &mut Affector) {
        for action in actions.iter() {
            action.execute(affector);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn processor_can_do_simple_action() {
        let mut world = World::new(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash));
        match world.get_entity(0, 0) {
            Entity::Cell(old_hash) => assert_eq!(*old_hash, hash),
            _ => panic!()
        }

        let kill_action = KillAction::new(0, 0);
        let mut list = LinkedList::new();
        list.push_back(kill_action);
        Processor::apply(&list, &mut world);

        let new_entity = world.get_entity(0, 0);
        match new_entity {
            Entity::Corpse(_) =>  {},
            _ => panic!()
        }
    }
}
