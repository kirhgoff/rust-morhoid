use crate::action::Action;
use crate::direction::Direction;
use crate::genome::{
    GenomeId, ATTACK, DEFILE, GENOME_LENGTH, MOVE, PHOTOSYNTHESIS, REPRODUCE, SENSE, TURN,
};
use crate::world::{Coords, Entity, World};

pub fn run_genome(world: &World, x: Coords, y: Coords, id: GenomeId) -> (Vec<Action>, usize) {
    let cell = world.cell(id);
    let genes = &cell.genome.genes;
    let settings = &world.settings;

    let mut actions: Vec<Action> = Vec::new();
    let mut index = cell.gene_index;

    for _ in 0..settings.steps_per_turn {
        let gene = genes[index];

        match gene {
            DEFILE => {
                actions.push(Action::Defile {
                    x,
                    y,
                    damage: settings.defile_damage,
                });
                index += 1
            }
            ATTACK => {
                actions.push(Action::Attack {
                    x,
                    y,
                    damage: settings.attack_damage,
                });
                index += 1
            }
            REPRODUCE => {
                actions.push(Action::Reproduce { x, y });
                index += 1
            }
            PHOTOSYNTHESIS => {
                actions.push(Action::UpdateHealth {
                    x,
                    y,
                    delta: settings.photosynthesis_adds,
                });
                index += 1
            }
            MOVE => {
                actions.push(Action::Move { x, y });
                index += 1
            }
            TURN => {
                let new_direction = genes[normalize_index(index + 1)] % Direction::SIZE;
                actions.push(Action::Rotate {
                    x,
                    y,
                    by: new_direction,
                });
                index += 2
            }
            SENSE => {
                actions.push(Action::UpdateHealth {
                    x,
                    y,
                    delta: settings.sense_cost,
                });
                let target = world
                    .looking_at(x, y)
                    .map(|(tx, ty)| world.entity_at(tx, ty));
                index = match target {
                    Some(Entity::Nothing) => genes[normalize_index(index + 1)],
                    Some(Entity::Cell(_)) => genes[normalize_index(index + 2)],
                    _ => index + 3,
                }
            }
            _ => {
                index = gene;
            }
        }

        if index >= GENOME_LENGTH {
            index = normalize_index(index)
        }
    }

    (actions, index)
}

fn normalize_index(index: usize) -> usize {
    index % GENOME_LENGTH
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::Genome;
    use crate::settings::Settings;

    #[test]
    fn integration_updates_genome_states() {
        let settings = Settings {
            steps_per_turn: 5,
            reproduce_cost: 0,
            attack_damage: 4,
            ..Settings::prod()
        };

        let mut world = World::new(2, 1, settings);

        let plant = Genome::new_plant();
        let hash = plant.id;
        world.set_cell(0, 0, plant);

        let plant2 = Genome::new_plant();
        let hash2 = plant2.id;
        world.set_cell(1, 0, plant2);

        for i in 0..10 {
            world.tick();
            assert_eq!(world.cell(hash).gene_index, 5 * (i + 1));
            assert_eq!(world.cell(hash2).gene_index, 5 * (i + 1));
        }

        for _ in 0..GENOME_LENGTH {
            world.tick();
        }
    }

    #[test]
    fn sense_jumps_by_target() {
        let settings = Settings::zero();
        let mut world = World::new(2, 1, settings);

        let mut genome = Genome::new_plant();
        genome.mutate(0, SENSE);
        genome.mutate(1, 40);
        genome.mutate(2, 50);
        let hash = genome.id;
        world.set_cell_facing(0, 0, genome, Direction::East);

        world.set_nothing(1, 0);
        world.tick();
        assert_eq!(world.cell(hash).gene_index, 40);

        world.cells.get_mut(&hash).unwrap().gene_index = 0;
        world.set_cell(1, 0, Genome::new_plant());
        world.tick();
        assert_eq!(world.cell(hash).gene_index, 50);

        world.cells.get_mut(&hash).unwrap().gene_index = 0;
        world.set_corpse(1, 0, 10);
        world.tick();
        assert_eq!(world.cell(hash).gene_index, 3);
    }

    #[test]
    fn integration_test_update_health() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        world.set_cell(0, 0, plant);

        world.apply(&[Action::UpdateHealth {
            x: 0,
            y: 0,
            delta: -100,
        }]);

        match world.entity_at(0, 0) {
            Entity::Corpse(_) => {}
            _ => panic!("cell should be dead here"),
        }
    }
}
