use std::fmt;
use std::collections::LinkedList;

use morphoid::entity::Entity;
use morphoid::processor::Processor;
use morphoid::action::*;
use morphoid::genome_storage::GenomeStorage;
use morphoid::cell_state_storage::CellStateStorage;
use morphoid::genome::HashType;
use morphoid::cell_state::HealthType;
use morphoid::cell_state::CellState;
use morphoid::genome::Genome;

pub type Coords = u32;

pub struct World {
    width: Coords,
    height: Coords,
    entities: Vec<Entity>,
    genomes: GenomeStorage,
    cell_states: CellStateStorage
}

impl World {
    pub fn new(width:Coords, height:Coords) -> World {
        let entities = (0..width * height)
            .map(|_| Entity::Nothing)
            .collect();

        World {
            width: width,
            height: height,
            entities: entities,
            genomes: GenomeStorage::new(),
            cell_states: CellStateStorage::new()
        }
    }

    // TODO: synchronize
    fn tick<T : Action>(&mut self) {
        // TODO move to processor
        let mut actions: LinkedList<Box<T>> = LinkedList::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let entity = self.entities[idx];
                let action_batch = Processor::process_entity(entity, self);

                // TODO how to do it better (collect iterators?)
                for action in action_batch {
                    actions.push_back(action);
                }
            }
        }
        Processor::apply(&actions, self);
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
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity, initial_state: Option<CellState>);
    fn update_health(&mut self, x:Coords, y:Coords, health_delta: HealthType);
}


impl Affector for World {
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity, initial_state: Option<CellState>) {
        let index = self.get_index(x, y);
        match self.entities[index] {
            Entity::Cell(hash) => {
                self.cell_states.remove(hash);
                self.genomes.remove(hash); // TODO: should we?
            },
            _ => {}
        }
        match entity {
            Entity::Cell(hash) => {
                self.cell_states.put(hash, initial_state.unwrap());
            },
            _ => {}
        }
        self.entities[index] = entity;
    }

    fn update_health(&mut self, x:Coords, y:Coords, health_delta: HealthType) {
        match self.entities[self.get_index(x, y)] {
            Entity::Cell(hash) => {
                {
                    // TODO: probably need to make it more tolerant
                    let mut state = self.cell_states.get_mut(hash);
                    state.health += health_delta;
                }
                if self.cell_states.get(hash).health < 0 {
                    self.set_entity(x, y, Entity::Corpse(10), None);
                }
            },
            _ => {}
        }

    }
}

pub trait Perceptor {
    // TODO do I need this method?
    fn get_state_mut(&mut self, hash: HashType) -> &mut CellState;

    fn get_entity(&self, x:Coords, y:Coords) -> &Entity;
    fn get_state(&self, hash: HashType) -> &CellState;
    fn get_genome(&self, hash: HashType) -> &Genome;
}

impl Perceptor for World {
    fn get_entity(&self, x:Coords, y:Coords) -> &Entity {
        &self.entities[self.get_index(x, y)]
    }

    // TODO: should not be mut
    fn get_state_mut(&mut self, hash: HashType) -> &mut CellState {
        self.cell_states.get_mut(hash)
    }

    fn get_state(&self, hash: HashType) -> &CellState {
        self.cell_states.get(hash)
    }

    fn get_genome(&self, hash: HashType) -> &Genome {
        self.genomes.get(hash).unwrap() // TODO: return Option?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morphoid::genome::Genome;

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
