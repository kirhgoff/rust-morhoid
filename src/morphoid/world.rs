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

pub type Coords = i32;

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

    // TODO: synchronize?
    pub fn tick(&mut self, processor: &Processor) {
        // TODO move to processor?
        // TODO use linked list for performance
        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);

                let entity = self.entities[idx];
                let mut action_batch = processor.process_entity(x, y, entity, self);
                actions.append(&mut action_batch);
            }
        }
        processor.apply(&actions, self);
    }

    // TODO: simplify
    fn get_index(&self, x: Coords, y: Coords) -> usize {
        let x2 = if x < 0 {
            if x % self.width == 0 {
                0
            } else {
                self.width + (x % self.width)
            }
        } else {
            x % self.width
        };

        let y2 = if y < 0 {
            if y % self.height == 0 {
                0
            } else {
                self.height - (y % self.height) * (-1)
            }

        } else {
            y % self.height
        };

        (y2 * self.width + x2) as usize
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
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity, genome: Option<Genome>, initial_state: Option<CellState>);
    fn update_health(&mut self, x:Coords, y:Coords, health_delta: HealthType);
    fn build_child_genome_for(&mut self, parent_genome_id: HashType) -> Genome;
}


impl Affector for World {
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity, genome:Option<Genome>, initial_state: Option<CellState>) {
        let index = self.get_index(x, y);
        //println!("set_entity x: {:?} y: {:?} index={:?}", x, y, index);
        match self.entities[index] {
            Entity::Cell(hash) => {
                self.cell_states.remove(hash);
                self.genomes.remove(hash); // TODO: should we?
            },
            _ => {}
        }
        match entity {
            Entity::Cell(hash) => {
                self.genomes.put(genome.unwrap());
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
                    self.set_entity(x, y, Entity::Corpse(10), None, None);
                }
            },
            _ => {}
        }
    }

    fn build_child_genome_for(&mut self, parent_genome_id: HashType) -> Genome {
        let parent_genome = self.genomes.get(parent_genome_id);
        parent_genome.clone() // TODO: add mutation
    }
}

pub trait Perceptor {
    // TODO do I need this method?
    fn get_state_mut(&mut self, hash: HashType) -> &mut CellState;

    fn get_entity(&self, x:Coords, y:Coords) -> &Entity;
    fn get_state(&self, hash: HashType) -> &CellState;
    fn get_genome(&self, hash: HashType) -> &Genome;
    fn find_vacant_place_around(&self, x:Coords, y:Coords) -> Option<(Coords, Coords)>;
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
        self.genomes.get(hash)
    }

    fn find_vacant_place_around(&self, x: Coords, y: Coords) -> Option<(Coords, Coords)> {
        let results: Vec<(Coords, Coords)> = iproduct!(x-1..x+1, y-1..y+1)
            .into_iter()
            .filter(|(i,j)| {
                match self.get_entity(*i, *j) {
                    Entity::Nothing => true,
                    _ => false
                }
            })
            .collect();

        // TODO: why, why?
        match results.first() {
            Some(x) => Some(*x),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morphoid::genome::Genome;
    use morphoid::genome::REPRODUCE;
    use morphoid::settings::Settings;

    #[test]
    fn integration_test() {
        let processor = Processor::with_settings(Settings {
            steps_per_turn: 2,
            reproduce_cost: -8, // it will die after new born
            reproduce_threshold: 9, // it will reproduce on second step
            photosynthesys_adds: 5 // it will have 10 + 5 health after first step
        });
        let mut world = World::new(2, 1);
        let mut plant = Genome::new_plant();
        plant.mutate(1, REPRODUCE);
        let hash = plant.hash();

        // TODO: add new_xxx methods
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState { health: 10 }));
        world.set_entity(1, 0, Entity::Nothing, None, None);

        world.tick(&processor);

        // Checking old entity state
        let cell_state = world.get_state(hash);
        assert_eq!(cell_state.health, 10 + 5 - 8); // TODO: use settings to amend the values

        match world.get_entity(1, 0) {
            Entity::Cell(another_hash) => assert_ne!(*another_hash, hash),
            _ => panic!("New cell was not reproduced!")
        }
    }

    #[test]
    fn get_index_test() {
        let world = World::new(2, 1);

        assert_eq!(world.get_index(-2,0), 0);
        assert_eq!(world.get_index(-1,0), 1);
        assert_eq!(world.get_index(0,0), 0);
        assert_eq!(world.get_index(1,0), 1);
        assert_eq!(world.get_index(2,0), 0);

        assert_eq!(world.get_index(0,0), 0);
        assert_eq!(world.get_index(0,1), 0);
        assert_eq!(world.get_index(0,2), 0);
        assert_eq!(world.get_index(0,-2), 0);
        assert_eq!(world.get_index(0,-1), 0);

        assert_eq!(world.get_index(1,0), 1);
        assert_eq!(world.get_index(1,1), 1);
        assert_eq!(world.get_index(1,2), 1);
        assert_eq!(world.get_index(1,-2), 1);
        assert_eq!(world.get_index(1,-1), 1);

    }
}
