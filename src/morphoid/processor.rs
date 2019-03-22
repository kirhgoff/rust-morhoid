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
                //println!("DEBUG: Processor.process_entity [cell] genome: {:?}", genome_id);
                let mut actions = self.execute(x, y, genome_id, perceptor, settings);
                all_actions.append(&mut actions);
            }
            _ => {
                //println!("DEBUG: Processor.process_entity [other] {:?}", otherwise);
            },
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

        let genome = perceptor.get_genome(genome_id).unwrap(); // should never happen
        let start_index = self.get_genome_index(genome_id);

        let mut index = start_index;
        for _ in 0..settings.steps_per_turn() {
//            println!("DEBUG: Processor.execute gene: {:?} index={:?}", genome_id, index);
            let gene = genome.genes[index];
            match gene {
                ATTACK => {
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.attack_cost())));
                    actions.push(Box::new(AttackAction::new(x, y, settings.attack_damage())))
                },
                REPRODUCE => {
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.reproduce_cost())));
                    actions.push(Box::new(ReproduceAction::new(new_x, new_y, genome_id)));
                },
                PHOTOSYNTHESYS => {
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.photosynthesys_adds())));
                },
                MOVE => {
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.move_cost())));
                    actions.push(Box::new(MoveAction::new(x, y)));
                },
                TURN => {
                    new_direction = genome.genes[index += 1] % Direction::SIZE;
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.turn_cost())));
                    actions.push(Box::new(RotateAction::new(x, y, new_direction)));
                },
                SENSE => {
                    actions.push(Box::new(UpdateHealthAction::new(x, y, settings.sense_cost())));

                    let (target_x, target_y) = perceptor.looking_at(x, y, genome_id);
                    match perceptor.get_entity(target_x, target_y) {
                        Entity::Nothing => {},
                        Entity::Cell(other_genome_id) => {},
                        Entity::Corpse(_) => {}
                        // TODO: current!!
                    }
                }
                _ => {
                    println!("Unknown gene: {}", gene);
                }
            }

            index += 1;
            if index >= GENE_LENGTH {
                index = index % GENE_LENGTH
            }
        }
        self.update_genome_index(genome_id, index);

//        println!("DEBUG: Processor.execute gene: {:?} start: {:?} steps: {:?} end: {:?}",
//                 genome_id, start_index, settings.steps_per_turn(), index);

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

    fn update_genome_index(&mut self, genome_id: GenomeId, new_index: GeneIndex)  {
        let genome_state = self.get_genome_state(genome_id);
        //let old_index = genome_state.current_gene;
        genome_state.current_gene = new_index;

//        println!("DEBUG: Processor.update_genome_index gene: {:?} old: {:?} new: {:?}",
//            genome_id, old_index, new_index);
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

        // make sure it is going around the genes array and not crash
        for _ in 0..GENE_LENGTH {
            world.tick(&mut processor);
        }
    }

    #[test]
    fn integration_can_do_kill_entity_action() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.hash();
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState::default()));

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
        world.set_entity(0, 0, Entity::Cell(hash), Some(plant), Some(CellState::default()));

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
