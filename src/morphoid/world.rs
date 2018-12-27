use std::fmt;

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
    fn tick(&mut self, processor: &Processor) {
        // TODO move to processor?
        // TODO use linked list for performance
        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let entity = self.entities[idx];
                let mut action_batch = processor.process_entity(entity, self);
                actions.append(&mut action_batch);
            }
        }
        processor.apply(&actions, self);
    }

    fn get_index(&self, x: Coords, y: Coords) -> usize {
        (y * self.width + x) as usize
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
        println!("set_entity x: {:?} y: {:?} index={:?}", x, y, index);
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
    fn integration_test() {
        let processor = Processor::new();
        let mut world = World::new(2, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();

        world.set_entity(0, 0, Entity::Cell(hash), Some(CellState { health: 10 }));
        world.set_entity(1, 0, Entity::Nothing, None);

        // Settings: sun power = 5
        // new baby born: 20
        world.tick(&processor);
        world.tick(&processor);
        world.tick(&processor);

        let new_entity = world.get_entity(0, 0);
        let cell_state = world.get_state(hash);
        assert_eq!(cell_state.health, 25);
    }

    #[test]
    fn get_index_test() {
        let mut world = World::new(2, 1);
        assert_eq!(world.get_index(0,0), 0);
        assert_eq!(world.get_index(1,0), 1);
    }
}
