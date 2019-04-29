extern crate time;

use std::fmt;
use morphoid::types::*;
//use self::time::PreciseTime;
use std::vec::Vec;

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
        //let start_time = PreciseTime::now();

        // TODO move to processor?
        // TODO use linked list for performance
        let mut actions: Vec<Box<dyn Action>> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_index(x, y);
                let entity = self.entities[idx];

                //println!("DEBUG: World.tick x: {:?} y: {:?} idx: {:?}", x, y, idx);
                let mut action_batch = processor.process_entity(x, y, entity, self, &self.settings);
                actions.append(&mut action_batch);
            }
        }
        processor.apply(&actions, self);

        // whatever you want to do
        //let end_time = PreciseTime::now();

        //println!("DEBUG World.tick actions: {:?} time: {:?}", actions.len(), start_time.to(end_time));
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
            Entity::Cell(genome.id()),
            Some(genome),
            Some(CellState::new(initial_health, Direction::North))
        );
    }

    fn move_cell(&mut self, x:Coords, y:Coords) {
        let old_index = self.get_index(x, y);

        match self.entities[old_index] {
            Entity::Cell(hash) => {
                let (new_x, new_y) = self.looking_at(x, y, hash);
                let new_index = self.get_index(new_x, new_y);

                match self.entities[new_index] {
                    Entity::Nothing => {
                        self.entities[new_index] = Entity::Cell(hash);
                        self.entities[old_index] = Entity::Nothing;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn rotate_cell(&mut self, x:Coords, y:Coords, value: Gene) {
        let index = self.get_index(x, y);

        match self.entities[index] {
            Entity::Cell(hash) => {
                let mut cell_state = self.cell_states.get_mut(hash);
                cell_state.direction = cell_state.direction.rotate(value);
            }
            _ => {}
        }
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

    // TODO: CURRENT - decide what to return in case of positive delta

    /// Returns positive amount of health bitten from target
    ///
    /// # Arguments
    ///
    /// * `x`
    /// * `y`
    /// * `health_delta` - will be added to cell's health

    fn update_health(&mut self, x:Coords, y:Coords, health_delta: HealthType) -> HealthType {
        let old_health;
        let new_health;

        let mut result = -health_delta;

        match self.entities[self.get_index(x, y)] {
            Entity::Cell(genome_id) => {
                // TODO: understand, still awkward
                {
                    let mut state = self.cell_states.get_mut(genome_id);
                    old_health = state.health;

                    state.health += health_delta;
                    new_health = state.health;

                    println!("DEBUG: Affector.update_health x={:?} y={:?} genome_id={:?} delta={:?} new_health={:?}",
                             x, y, genome_id, health_delta, state.health);
                }

                if new_health < 0 {
                    result = old_health;
                    self.set_entity(x, y, Entity::Corpse(666), None, None);
                }
            },
            _ => {}
        }
        result
    }

    fn attack(&mut self, x:Coords, y:Coords, damage: HealthType) {
        let index = self.get_index(x, y);

        match self.entities[index] {
            Entity::Cell(hash) => {
                let (new_x, new_y) = self.looking_at(x, y, hash);
                let health_eaten = self.update_health(new_x, new_y, -damage);

                println!("DEBUG: Affector.attack x: {:?} y: {:?} new_x: {:?}, new_y: {:?} health_eaten: {:?}",
                         x, y, new_x, new_y, health_eaten);

                self.update_health(x, y, health_eaten);
            }
            other => {
                println!("DEBUG: Affector.attack other: {:?}", other)
            }
        }
    }

    fn build_child_genome_for(&mut self, parent_genome_id: GenomeId) -> Option<Genome> {
        self.genomes
            .get(parent_genome_id)
            .map(|genome| genome.clone()) // TODO: mutate
    }
}


impl Perceptor for World {
    fn get_entity(&self, x:Coords, y:Coords) -> &Entity {
        &self.entities[self.get_index(x, y)]
    }

    fn get_state(&self, genome_id: GenomeId) -> &CellState {
        self.cell_states.get(genome_id)
    }

    fn get_state_by_pos(&self, x:Coords, y:Coords) -> Option<&CellState> {
        match self.get_entity(x, y) {
            Entity::Cell(genome_id) => Some(self.get_state(*genome_id)),
            _ => None
        }
    }

    fn get_genome(&self, hash: GenomeId) -> Option<&Genome> {
        self.genomes.get(hash)
    }

    fn looking_at(&self, x: i32, y: i32, hash: u64) -> (Coords, Coords) {
        let cell_state = self.cell_states.get(hash);
        let (dx, dy) = cell_state.direction.shift();
        (x + dx, y + dy)
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
        iproduct!(-1..2, -1..2)
            .into_iter()
            .filter(|(dx, dy)| *dx != 0 || *dy != 0) // remove self
            .map(|(dx, dy)| (dy, dx)) // turn around so it is clockwise
            .map(|(dx, dy)| (x + dx, y + dy))
            .filter(|(other_x, other_y)| {
                match self.get_entity(*other_x, *other_y) {
                    Entity::Cell(_) =>  true,
                    _ => false
                }
            })
            .collect::<Vec<(Coords,Coords)>>()
            .first() // TODO: randomize
            .map(|coords| *coords)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index_test() {
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

    #[test]
    fn test_find_target_around() {
        let mut world = World::new(3, 3, Settings::prod());
        for x in 0..3 {
            for y in 0..3 {
                world.set_cell(x, y, Genome::new_predator());
            }
        }

        assert_eq!(world.find_target_around(1, 1), Some((0,0)));

        world.set_nothing(0, 0);
        assert_eq!(world.find_target_around(1, 1), Some((1,0)));

        world.set_nothing(1, 0);
        assert_eq!(world.find_target_around(1, 1), Some((2,0)));

        world.set_nothing(2, 0);
        assert_eq!(world.find_target_around(1, 1), Some((0,1)));
    }

    #[test]
    fn test_update_health_addition() {
        let settings = Settings::prod();
        let initial_cell_health = settings.initial_cell_health();

        let mut world = World::new(1, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());

        // we always receive negative value from update
        assert_eq!(world.update_health(0, 0, 10), - 10);

        assert_eq!(
            world.get_state_by_pos(0, 0).expect("There should be cell here").health,
            initial_cell_health + 10
        );
    }

    #[test]
    fn test_update_health_small_damage() {
        let settings = Settings::prod();
        let initial_cell_health = settings.initial_cell_health();

        let mut world = World::new(1, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());

        assert_eq!(
            world.update_health(0, 0, - initial_cell_health + 5),
            initial_cell_health - 5
        );

        assert_eq!(
            world.get_state_by_pos(0, 0).expect("There should be cell here").health,
            5
        );
    }

    #[test]
    fn test_update_health_big_damage() {
        let settings = Settings::prod();
        let initial_cell_health = settings.initial_cell_health();

        let mut world = World::new(1, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());

        assert_eq!(
            world.update_health(0, 0, - initial_cell_health - 1),
            initial_cell_health // You can ony eat what is there
        );

        match world.get_entity(0, 0) {
            Entity::Cell(_) => panic!("Cell should be dead!"),
            _ => {}
        }
    }

    // TODO: duplicate tests in actions
    #[test]
    fn integration_test_it_reproduces() {
        let settings =
            SettingsBuilder::prod()
                .with_reproduce_threshold(9) // it will reproduce on second step
                .with_photosynthesys_adds(5) // it will have 10 + 5 health after first step
                .with_initial_cell_health(10)// it will have 10 originally
                .build();

        let new_value =
            settings.initial_cell_health() +
            settings.photosynthesys_adds() +
            settings.reproduce_cost();

        let mut processor = Processor::new();
        let mut world = World::new(2, 1, settings);
        let mut plant = Genome::new_plant();
        plant.mutate(1, REPRODUCE);
        let hash = plant.id();

        world.set_cell(0, 0, plant);
        world.set_nothing(1, 0);

        world.tick(&mut processor);

        // Checking old entity state
        let cell_state = world.get_state(hash);
        assert_eq!(cell_state.health, new_value);

        match world.get_entity(1, 0) {
            Entity::Cell(another_hash) => assert_ne!(*another_hash, hash),
            _ => panic!("New cell was not reproduced!")
        }
    }

    #[test]
    fn integration_test_and_then_there_were_none() {
        let settings = Settings::prod();
        let mut processor = Processor::new();
        let mut world = World::new(3, 3, settings);

        // set the scene, killer in the middle
        for x in 0..3 {
            for y in 0..3 {
                if x != 1 || y != 1 {
                    world.set_cell(x, y, Genome::new_plant());
                }
            }
        }
        world.set_cell(1, 1, Genome::new_predator());

        // One shot kills
        for _ in 0..9 {
            world.tick(&mut processor)
        }

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
    fn integration_test_plant_reproduce_if_have_enough() {
        let settings = SettingsBuilder::prod()
            .with_reproduce_cost(0)
            .with_reproduce_threshold(4) // it will reproduce on first step
            .build(); // it will have 10 originally

        let mut processor = Processor::new();
        let mut world = World::new(2, 1, settings);
        let mut plant = Genome::new_plant();
        plant.mutate(1, REPRODUCE);
        let hash = plant.id();

        world.set_cell(0, 0, plant);
        world.set_nothing(1, 0);

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

    #[test]
    fn integration_test_order_of_execution_parent_killed() {
        let mut world = World::new(3, 1, SettingsBuilder::zero());
        let parent = Genome::new_yeast();
        let parent_genome_id = parent.id();

        world.set_cell(1, 0, parent);
        world.set_cell(2, 0, Genome::new_predator());

        Processor::new().apply(
            &vec![
                Box::new(AttackAction::new(2, 0, 100)),
                Box::new(ReproduceAction::new(0, 0, parent_genome_id))
            ],
            &mut world
        );

        // attack came first, sad
        match world.get_entity(1, 0) {
            Entity::Corpse(_) => {},
            _ => panic!("Parent should have been destroyed")
        }

        match world.get_entity(0, 0) {
            Entity::Nothing => {},
            _ => panic!("Nothing should be born")
        }
    }

    #[test]
    fn integration_test_order_of_execution_parent_gives_birth() {
        let settings = SettingsBuilder::zero();

        let mut world = World::new(3, 1, settings);
        let parent = Genome::new_yeast();
        let parent_genome_id = parent.id();

        world.set_cell(1, 0, parent);
        world.set_cell(2, 0, Genome::new_predator());

        Processor::new().apply(
            &vec![
                Box::new(ReproduceAction::new(0, 0, parent_genome_id)),
                Box::new(AttackAction::new(2, 0, 100))
            ],
            &mut world
        );

        // attack came first, sad
        match world.get_entity(1, 0) {
            Entity::Corpse(_) => {},
            _ => panic!("Parent should have been destroyed")
        }

        match world.get_entity(0, 0) {
            Entity::Cell(_) => {},
            _ => panic!("But new life survived!")
        }
    }
}
