extern crate time;

use std::fmt;
use morphoid::types::*;
use self::time::PreciseTime;

impl World {
    pub fn prod(width:Coords, height:Coords) -> World {
        World::new(width, height, Settings::prod())
    }

    pub fn new(width:Coords, height:Coords, settings: Settings) -> World {
        let entities = (0..width * height)
            .map(|_| Entity::Nothing)
            .collect();

        World {
            width: width,
            height: height,
            settings: settings,
            entities: entities,
            genomes: GenomeStorage::new(),
            cell_states: CellStateStorage::new()
        }
    }

    // TODO: synchronize?
    pub fn tick(&mut self, processor: &mut Processor) {
        let start_time = PreciseTime::now();

        // TODO move to processor?
        // TODO use linked list for performance
        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);

                let entity = self.entities[idx];
                let mut action_batch = processor.process_entity(x, y, entity, self, &self.settings);
                actions.append(&mut action_batch);
            }
        }
        processor.apply(&actions, self);

        // whatever you want to do
        let end_time = PreciseTime::now();

        println!("LOG World.tick actions: {:?} time: {:?}", actions.len(), start_time.to(end_time));
    }

    fn get_index(&self, x: Coords, y: Coords) -> usize {
        let x2 = World::normalize(x, self.width);
        let y2 = World::normalize(y, self.height);

        (y2 * self.width + x2) as usize
    }

    fn normalize(coord:Coords, dimension: Coords) -> Coords {
        let remainder = coord % dimension;
        if coord < 0 {
            if remainder == 0 { 0 } else { dimension + remainder }
        } else {
            remainder
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.entities.as_slice().chunks(self.width as usize) {
            for &entity in line {
                let symbol = match entity {
                    Entity::Nothing => '◻',
                    Entity::Cell(_) => '◼',
                    Entity::Corpse(_) => 'x',
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Affector for World {
    fn set_nothing(&mut self, x:Coords, y:Coords) {
        self.set_entity(
            x,
            y,
            Entity::Nothing,
            None,
            None
        );
    }

    fn set_cell(&mut self, x:Coords, y:Coords, genome:Genome) {
        let initial_health = self.settings.initial_cell_health;
        self.set_entity(
            x,
            y,
            Entity::Cell(genome.hash()),
            Some(genome),
            Some(CellState {health: initial_health })
        );
    }

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
                    //println!("World.update_health x={:?} y={:?} hash={:?} delta={:?} new_health={:?}",
                    //    x, y, hash, health_delta, state.health);
                }
                if self.cell_states.get(hash).health < 0 {
                    self.set_entity(x, y, Entity::Corpse(10), None, None);
                }
            },
            _ => {}
        }
    }

    fn build_child_genome_for(&mut self, parent_genome_id: GenomeId) -> Genome {
        let parent_genome = self.genomes.get(parent_genome_id);
        parent_genome.clone() // TODO: add mutation
    }
}

impl Perceptor for World {
    fn get_entity(&self, x:Coords, y:Coords) -> &Entity {
        &self.entities[self.get_index(x, y)]
    }

    // TODO: should not be mut
    fn get_state_mut(&mut self, hash: GenomeId) -> &mut CellState {
        self.cell_states.get_mut(hash)
    }

    fn get_state(&self, hash: GenomeId) -> &CellState {
        self.cell_states.get(hash)
    }

    fn get_genome(&self, hash: GenomeId) -> &Genome {
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

    // TODO: extract method
    fn find_target_around(&self, x: Coords, y: Coords) -> Option<(Coords, Coords)> {
        let results: Vec<(Coords, Coords)> = iproduct!(x-1..x+1, y-1..y+1)
            .into_iter()
            .filter(|(i,j)| {
                match self.get_entity(*i, *j) {
                    Entity::Cell(_) => (*i != x && *j != y),
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

    #[test]
    fn get_index_test() {
        let world = World::prod(2, 1);

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

    // TODO: duplicate tests in actions
    #[test]
    fn integration_test_it_reproduces() {
        let settings = Settings {
            steps_per_turn: 2,
            reproduce_cost: -8, // it will die after new born
            reproduce_threshold: 9, // it will reproduce on second step
            photosynthesys_adds: 5, // it will have 10 + 5 health after first step
            initial_cell_health: 10, // it will have 10 originally
            attack_damage: 4,
        };

        let mut processor = Processor::new();
        let mut world = World::new(2, 1, settings);
        let mut plant = Genome::new_plant();
        plant.mutate(1, REPRODUCE);
        let hash = plant.hash();

        world.set_cell(0, 0, plant);
        world.set_nothing(1, 0);

        world.tick(&mut processor);

        // Checking old entity state
        let cell_state = world.get_state(hash);
        assert_eq!(cell_state.health, 10 + 5 - 8); // TODO: use settings to amend the values

        match world.get_entity(1, 0) {
            Entity::Cell(another_hash) => assert_ne!(*another_hash, hash),
            _ => panic!("New cell was not reproduced!")
        }
    }

    #[test]
    fn integration_test_and_then_there_were_none() {
        let settings = Settings {
            steps_per_turn: 1,
            reproduce_cost: -10,
            reproduce_threshold: 20,
            photosynthesys_adds: 5,
            initial_cell_health: 10,
            attack_damage: 100,
        };

        let mut processor = Processor::new();
        let mut world = World::new(3, 3, settings);

        // set the scene, killer in the middle
        world.set_cell(1, 1, Genome::new_predator());
        for x in 0..2 {
            for y in 0..2 {
                if x != y {
                    world.set_cell(x, y, Genome::new_plant());
                }
            }
        }

        // One shot kills
        (1..9).map(|_| world.tick(&mut processor));

        // Should all be dead
        for x in 0..2 {
            for y in 0..2 {
                if x != y {
                    match world.get_entity(x, y) {
                        Entity::Corpse(_) => {},
                        _ => panic!("This guys is not dead!")
                    }
                }
            }
        }

        match world.get_entity(1, 1) {
            Entity::Cell(_) => {},
            _ => panic!("The killer must survive!")
        }
    }


    #[test]
    fn integration_test_genome_state() {
        let settings = Settings {
            steps_per_turn: 1,
            reproduce_cost: -0,
            reproduce_threshold: 4, // it will reproduce on first step
            photosynthesys_adds: 5, // it will have 10 + 5 health after first step
            initial_cell_health: 10, // it will have 10 originally
            attack_damage: 4,
        };

        let mut processor = Processor::new();
        let mut world = World::new(2, 1, settings);
        let mut plant = Genome::new_plant();
        plant.mutate(1, REPRODUCE);
        let hash = plant.hash();

        // TODO: add new_xxx methods
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState { health: 10 }));
        world.set_entity(1, 0, Entity::Nothing, None, None);

        world.tick(&mut processor);

        // Checking nothing is still nothing
        match world.get_entity(1, 0) {
            Entity::Nothing => {},
            _ => panic!("New cell was created! WTF?")
        }

        // Now it will reproduce
        world.tick(&mut processor);

        // Checking new cell has been born
        match world.get_entity(1, 0) {
            Entity::Cell(another_hash) => assert_ne!(*another_hash, hash),
            _ => panic!("New cell was not reproduced!")
        }

    }
}
