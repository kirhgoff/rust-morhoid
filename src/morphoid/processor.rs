use morphoid::types::*;
use std::collections::HashMap;

impl Processor {
    pub fn new() -> Processor {
        Processor { genome_states: HashMap::new() }
    }

    pub fn process_entity(&mut self, x:Coords, y:Coords, entity: Entity, perceptor: &Perceptor, settings: &Settings) -> Vec<Box<dyn Action>> {
        let mut all_actions:Vec<Box<dyn Action>> = Vec::new();
        match entity {
            Entity::Cell(genome_id) => {
                let mut actions = self.execute(x, y, genome_id, perceptor, settings);
                all_actions.append(&mut actions);
            }
            _ => {},
        };
        all_actions
    }

    // TODO: move to world
    pub fn apply(&self, actions: &Vec<Box<dyn Action>>, affector: &mut Affector) {
        for action in actions.iter() {
            action.execute(affector);
        }
    }

    pub fn execute(&mut self, x:Coords, y:Coords, genome_id: GenomeId, perceptor: &Perceptor, settings: &Settings) -> Vec<Box<dyn Action>> {
        let mut actions:Vec<Box<dyn Action>> = Vec::new();

        let genome = perceptor.get_genome(genome_id);

        let start_index = self.get_genome_index(genome_id);
        let steps_limit = settings.steps_per_turn();
        let end_index = (start_index + steps_limit) % GENE_LENGTH;

        println!("LOG: Processor.execute start: {:?} steps: {:?} end: {:?}",
            start_index, steps_limit, end_index);

        let mut index_delta = 0;
        for i in start_index..end_index {
            println!("DEBUG Processor.execute gene index={:?}", i);
            let gene = genome.genes[i];
            match gene {
                ATTACK => {
                    match perceptor.find_target_around(x, y) {
                        Some((victim_x, victim_y)) => {
                            actions.push(Box::new(
                                AttackAction::new(victim_x, victim_y, x, y, settings.attack_damage()))
                            );
                            index_delta += 1;
                        },
                        _ => {}
                    }
                },
                REPRODUCE => {
                    // TODO: move all to action?
                    if perceptor.get_state(genome_id).health > settings.reproduce_threshold() {
                        actions.push(Box::new(UpdateHealthAction::new(x, y, settings.reproduce_cost())));
                        match perceptor.find_vacant_place_around(x, y) {
                            Some((new_x, new_y)) => {
                                //TODO extract to a method
                                actions.push(Box::new(ReproduceAction::new(new_x, new_y, genome_id)));
                                index_delta += 1;
                            },
                            _ => {}
                        }
                    }
                }, // 30
                PHOTOSYNTHESYS => {
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.photosynthesys_adds())));
                    index_delta += 1;
                }, // 31
                _ => {
                    println!("Unknown gene");
                }
            }
        }
        self.update_genome_index(genome_id, index_delta);
        //println!(">>>>>>>>> New index: {:?}", self.get_genome_index(genome_id));
        actions
    }

    fn get_genome_state(&mut self, genome_id: GenomeId) -> &mut GenomeState {
        self.genome_states
            .entry(genome_id)
            .or_insert(GenomeState { current_gene: 0 })
    }

    fn get_genome_index(&mut self, genome_id: GenomeId) -> GeneIndex {
        self.get_genome_state(genome_id).current_gene
    }

    fn update_genome_index(&mut self, genome_id: GenomeId, index_delta: GeneIndex)  {
        let genome_state = self.get_genome_state(genome_id);
        let current_index = genome_state.current_gene;
        let new_current_index = Processor::normalize(current_index, index_delta);
        genome_state.current_gene = new_current_index;

        println!("DEBUG Processor.update_genome_index current: {:?} delta: {:?} new_current: {:?}",
            current_index, index_delta, new_current_index);
    }

    fn normalize(curent_index:GeneIndex, delta: GeneIndex) -> GeneIndex {
        let mut new_current_index = curent_index + delta;
        if new_current_index > GENE_LENGTH {
            new_current_index = (new_current_index % GENE_LENGTH) as GeneIndex;
        }
        new_current_index
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn integration_updates_genome_states() {
        let settings = Settings {
            steps_per_turn: 5,
            reproduce_cost: -0,
            reproduce_threshold: 4, // it will reproduce on first step
            photosynthesys_adds: 5, // it will have 10 + 5 health after first step
            initial_cell_health: 10, // it will have 10 originally
            attack_damage: 4,
        };

        let mut processor = Processor::new();
        let mut world = World::new(2, 1, settings);

        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_cell(0, 0, plant);

        let plant2 = Genome::new_plant();
        let hash2 = plant2.hash();
        world.set_cell(1, 0, plant2);

        for i in 0..10 {
            world.tick(&mut processor);
            assert_eq!(processor.get_genome_index(hash), 5 * (i + 1));
            assert_eq!(processor.get_genome_index(hash2), 5 * (i + 1));
        }
    }

    #[test]
    fn integration_can_do_kill_entity_action() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState{health: 10}));

        match world.get_entity(0, 0) {
            Entity::Cell(old_hash) => assert_eq!(*old_hash, hash),
            _ => panic!()
        }

        Processor::new().apply(
            &vec![Box::new(KillAction::new(0, 0))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Corpse(_) =>  {},
            _ => panic!()
        }
    }

    #[test]
    fn integration_can_do_update_health() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState { health: 10 }));

        Processor::new().apply(
            &vec![Box::new(UpdateHealthAction::new(0, 0, -100))],
            &mut world
        );

        match world.get_entity(0, 0) {
            Entity::Corpse(_) =>  {},
            _ => panic!("Cell should be dead here")
        }
    }
}
