use std::fmt;
use std::collections::LinkedList;
use std::result::Iter;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(i64),
}

pub struct World {
    width: u32,
    height: u32,
    entities: Vec<Entity>,
}

impl World {
    pub fn new(width:u32, height:u32) -> World {
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
        let mut new_entities = self.entities.clone();
        // TODO move to processor
        let mut actions: LinkedList<T> = LinkedList::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let entity = self.entities[idx];
                let (new_entity, action_batch) = Processor::new_entity(entity, self);

                new_entities[idx] = new_entity;

                // TODO how to do it better (collect iterators?)
                for action in action_batch {
                    actions.push_back(action);
                }

                Processor::apply(&new_entities, &actions, self);
            }
        }

        self.entities = new_entities;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
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

trait Action : Sized {
    // do something with stats or replace with dirt
}

struct Processor {}

impl Processor {
    fn new_entity<T : Action>(entity: Entity, perceptor: &Perceptor) -> (Entity, Vec<T>) {
        let new_entity = match entity {
            Entity::Cell(gene_id) => Entity::Cell(gene_id + 1),
            otherwise => otherwise,
        };
        (new_entity, vec![])
    }

    fn apply<T : Action>(mut entities: &Vec<Entity>, actions: &LinkedList<T>, affector: &Affector) {

    }
}

trait Affector {}

trait Perceptor {}

impl Perceptor for World {}
impl Affector for World {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_load_dictionary_and_translate() {
//            let path = b"tests/locales/\0".as_ptr() as *const libc::c_char;
//            librhino_initialize(path);
//
//            let locale = b"en-IE\0".as_ptr() as *const libc::c_char;
//            let key = b"base.search_form.location_caption\0".as_ptr() as *const libc::c_char;
//            let result: *const libc::c_char = librhino_translate(locale, key);
//
//            let str_slice: &str = CStr::from_ptr(result).to_str().unwrap();
//            assert_eq!(str_slice, "City, county or province");
    }
}