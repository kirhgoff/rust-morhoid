use crate::genome::Gene;
use crate::world::{Coords, HealthType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    UpdateHealth {
        x: Coords,
        y: Coords,
        delta: HealthType,
    },
    Reproduce {
        x: Coords,
        y: Coords,
    },
    Attack {
        x: Coords,
        y: Coords,
        damage: HealthType,
    },
    Move {
        x: Coords,
        y: Coords,
    },
    Rotate {
        x: Coords,
        y: Coords,
        by: Gene,
    },
    Defile {
        x: Coords,
        y: Coords,
        damage: HealthType,
    },
    Decay {
        x: Coords,
        y: Coords,
        amount: HealthType,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;
    use crate::genome::{Genome, MOVE, TURN};
    use crate::settings::Settings;
    use crate::world::{Entity, World};

    #[test]
    fn test_update_health() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let hash = plant.id;
        world.set_cell(0, 0, plant);

        assert_eq!(world.cell(hash).health, 10);

        world.apply(&[Action::UpdateHealth {
            x: 0,
            y: 0,
            delta: 5,
        }]);

        assert_eq!(world.cell(hash).health, 15);
    }

    #[test]
    fn test_reproduce() {
        let mut world = World::prod(2, 1);
        let plant = Genome::new_plant();
        let genome_id = plant.id;
        world.set_cell_facing(0, 0, plant, Direction::East);

        world.apply(&[Action::Reproduce { x: 0, y: 0 }]);

        match world.entity_at(1, 0) {
            Entity::Cell(new_hash) => {
                assert_ne!(new_hash, genome_id);
                assert_eq!(world.cell(new_hash).health, 10);
            }
            _ => panic!("cant find reproduced entity"),
        }
    }

    #[test]
    fn test_attack() {
        let settings = Settings::zero();
        let new_value = settings.initial_cell_health + settings.attack_cost;
        let eaten = settings.initial_cell_health;

        let mut world = World::new(2, 1, settings);
        world.set_cell_facing(0, 0, Genome::new_plant(), Direction::East);
        world.set_cell_facing(1, 0, Genome::new_predator(), Direction::West);

        world.apply(&[Action::Attack {
            x: 1,
            y: 0,
            damage: 100,
        }]);

        match world.entity_at(0, 0) {
            Entity::Corpse(_) => {}
            _ => panic!("cell survived after 100 of damage"),
        }

        match world.entity_at(1, 0) {
            Entity::Cell(genome_id) => {
                assert_eq!(world.cell(genome_id).health, new_value + eaten);
            }
            _ => panic!("predator cell should have high health"),
        }
    }

    #[test]
    fn test_attack_nothing_gives_no_health() {
        let settings = Settings::zero();
        let initial_health = settings.initial_cell_health;

        let mut world = World::new(2, 1, settings);
        world.set_nothing(0, 0);
        world.set_cell_facing(1, 0, Genome::new_predator(), Direction::West);

        world.apply(&[Action::Attack {
            x: 1,
            y: 0,
            damage: 100,
        }]);

        match world.entity_at(1, 0) {
            Entity::Cell(genome_id) => {
                assert_eq!(world.cell(genome_id).health, initial_health)
            }
            _ => panic!("predator should still be here"),
        }
    }

    #[test]
    fn test_move() {
        let mut world = World::prod(2, 1);
        let mut plant = Genome::new_plant();
        plant.mutate(0, MOVE);

        let genome_id = plant.id;

        world.set_cell_facing(0, 0, plant, Direction::East);

        world.apply(&[Action::Move { x: 0, y: 0 }]);

        match world.entity_at(0, 0) {
            Entity::Nothing => {}
            _ => panic!("cell should have moved away"),
        }

        match world.entity_at(1, 0) {
            Entity::Cell(new_hash) => {
                assert_eq!(new_hash, genome_id);
            }
            _ => panic!("cell should have moved in"),
        }
    }

    #[test]
    fn test_rotate() {
        let mut world = World::prod(1, 1);
        let mut plant = Genome::new_plant();
        plant.mutate(0, TURN);
        plant.mutate(1, 1);

        let genome_id = plant.id;

        world.set_cell_facing(0, 0, plant, Direction::North);

        world.apply(&[Action::Rotate { x: 0, y: 0, by: 1 }]);

        match world.entity_at(0, 0) {
            Entity::Cell(new_genome_id) => {
                let cell_state = world.cell(new_genome_id);
                assert_eq!(Direction::NorthEast, cell_state.direction);
                assert_eq!(genome_id, new_genome_id);
            }
            _ => panic!("cell should still be here"),
        }
    }

    #[test]
    fn test_defile() {
        let settings = Settings {
            initial_cell_health: 10,
            corpse_initial: 10,
            defile_damage: 5,
            defile_cost: -3,
            ..Settings::prod()
        };

        let new_health =
            settings.initial_cell_health + settings.defile_cost + settings.defile_damage;

        let mut world = World::new(2, 1, settings);
        world.set_cell_facing(0, 0, Genome::new_predator(), Direction::East);
        world.set_corpse(1, 0, 10);

        world.apply(&[Action::Defile {
            x: 0,
            y: 0,
            damage: 5,
        }]);

        match world.entity_at(0, 0) {
            Entity::Cell(genome_id) => {
                assert_eq!(new_health, world.cell(genome_id).health);
            }
            _ => panic!("there should be a predator"),
        }

        match world.entity_at(1, 0) {
            Entity::Corpse(remains) => {
                assert_eq!(5, remains)
            }
            _ => panic!("corpse should stay here"),
        }
    }

    #[test]
    fn test_decay() {
        let mut world = World::prod(1, 1);

        world.set_corpse(0, 0, 10);

        world.apply(&[Action::Decay {
            x: 0,
            y: 0,
            amount: -3,
        }]);

        match world.entity_at(0, 0) {
            Entity::Corpse(remains) => assert_eq!(7, remains),
            _ => panic!("corpse should be here"),
        }
    }
}
