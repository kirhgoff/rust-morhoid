use morphoid::action::*;
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

        let mut index_delta = 0;
        for i in start_index..end_index {
            let gene = genome.genes[i];
            match gene {
                REPRODUCE => {
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
                _ => {}
            }
        }
        self.update_genome_index(genome_id, index_delta);
        println!(">>>>>>>>> New index: {:?}", self.get_genome_index(genome_id));
        actions
    }

    fn get_genome_index(&mut self, genome_id: GenomeId) -> GeneIndex {
        let genome_state = self.genome_states
            .entry(genome_id)
            .or_insert(GenomeState { current_gene: 0 });

        genome_state.current_gene
    }

    fn update_genome_index(&mut self, genome_id: GenomeId, index_delta: GeneIndex)  {
        let genome_state = self.genome_states
            .entry(genome_id)
            .or_insert(GenomeState { current_gene: 0 });

        genome_state.current_gene = Processor::normalize(genome_state.current_gene, index_delta)
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
    fn processor_updates_genome_states() {
        let settings = Settings {
            steps_per_turn: 1,
            reproduce_cost: -0,
            reproduce_threshold: 4, // it will reproduce on first step
            photosynthesys_adds: 5, // it will have 10 + 5 health after first step
            initial_cell_health: 10, // it will have 10 originally
        };

        let mut processor = Processor::new();
        let mut world = World::new(1, 1, settings);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.new_plant(0, 0, plant);

        world.tick(&mut processor);

        assert_eq!(processor.get_genome_index(hash), 1);
    }


    #[test]
    fn processor_can_do_kill_entity_action() {
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
    fn processor_can_do_update_health() {
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
