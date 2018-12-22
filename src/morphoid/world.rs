use std::fmt;
use std::collections::LinkedList;

use morphoid::entity::Entity;
use morphoid::processor::Processor;
use morphoid::action::Action;

pub type Coords = u32;

pub struct World {
    width: Coords,
    height: Coords,
    entities: Vec<Entity>,
}

impl World {
    pub fn new(width:Coords, height:Coords) -> World {
        let entities = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Entity::Cell(1)
                } else {
                    Entity::Nothing
                }
            })
            .collect();
        World {width, height, entities: entities}
    }

    // TODO: synchronize
    fn tick<T : Action>(&mut self) {
        // TODO move to processor
        let mut actions: LinkedList<T> = LinkedList::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let entity = self.entities[idx];
                //TODO: no need to return entities
                let (_, action_batch) = Processor::new_entity(entity, self);

                // TODO how to do it better (collect iterators?)
                for action in action_batch {
                    actions.push_back(action);
                }

                Processor::apply(&actions.iter(), self);
            }
        }
    }

    fn get_index(&self, row: Coords, column: Coords) -> usize {
        (row * self.width + column) as usize
    }
}


impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.entities.as_slice().chunks(self.width as usize) {
            for &entity in line {
                let symbol = if entity == Entity::Nothing { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

pub trait Affector {
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity);
}

impl Affector for World {
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity) {
        let index = self.get_index(x, y);
        self.entities[index] = entity;
    }
}

pub trait Perceptor {
    fn get_entity(&self, x:Coords, y:Coords) -> &Entity;
}

impl Perceptor for World {
    fn get_entity(&self, x:Coords, y:Coords) -> &Entity {
        &self.entities[self.get_index(x, y)]
    }
}
