use morphoid::entity::Entity;
use std::collections::linked_list::LinkedList;
use morphoid::action::Action;
use morphoid::world::Perceptor;
use morphoid::world::Affector;

pub struct Processor {}

impl Processor {
    pub fn new_entity<T : Action>(entity: Entity, perceptor: &Perceptor) -> (Entity, Vec<T>) {
        let mut actions:Vec<Action> = vec![];
        let new_entity = match entity {
            Entity::Cell(gene_id) => Entity::Cell(gene_id + 1),
            otherwise => otherwise,
        };
        (new_entity, vec![])
    }

    pub fn apply<T : Action>(mut entities: &Vec<Entity>, actions: &LinkedList<T>, affector: &Affector) {

    }
}
