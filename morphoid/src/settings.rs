use crate::genome::{Gene, ATTACK, DEFILE, MOVE, REPRODUCE, SENSE, TURN};
use crate::world::HealthType;

#[derive(Debug, Clone)]
pub struct Settings {
    pub steps_per_turn: usize,
    pub reproduce_cost: HealthType,
    pub photosynthesis_adds: HealthType,
    pub initial_cell_health: HealthType,
    pub attack_damage: HealthType,
    pub defile_damage: HealthType,
    pub attack_cost: HealthType,
    pub move_cost: HealthType,
    pub turn_cost: HealthType,
    pub sense_cost: HealthType,
    pub defile_cost: HealthType,
    pub corpse_decay: HealthType,
    pub corpse_initial: HealthType,
    pub mutation_probability: f64,
}

impl Settings {
    pub fn prod() -> Settings {
        Settings {
            steps_per_turn: 1,
            reproduce_cost: -10,
            photosynthesis_adds: 5,
            initial_cell_health: 10,
            attack_damage: 100,
            defile_damage: 10,
            attack_cost: -5,
            move_cost: -5,
            turn_cost: -5,
            sense_cost: -5,
            defile_cost: -1,
            corpse_decay: -2,
            corpse_initial: 20,
            mutation_probability: 0.5,
        }
    }

    pub fn zero() -> Settings {
        Settings {
            steps_per_turn: 1,
            reproduce_cost: 0,
            photosynthesis_adds: 0,
            initial_cell_health: 10,
            attack_damage: 0,
            defile_damage: 0,
            attack_cost: 0,
            move_cost: 0,
            turn_cost: 0,
            sense_cost: 0,
            defile_cost: 0,
            corpse_decay: 0,
            corpse_initial: 0,
            mutation_probability: 0.0,
        }
    }

    pub fn cost_of(&self, gene: Gene) -> HealthType {
        match gene {
            SENSE => self.sense_cost,
            TURN => self.turn_cost,
            MOVE => self.move_cost,
            ATTACK => self.attack_cost,
            REPRODUCE => self.reproduce_cost,
            DEFILE => self.defile_cost,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::PHOTOSYNTHESIS;

    #[test]
    fn cost_of_uses_prod_costs() {
        let settings = Settings::prod();

        assert_eq!(settings.sense_cost, settings.cost_of(SENSE));
        assert_eq!(settings.turn_cost, settings.cost_of(TURN));
        assert_eq!(settings.move_cost, settings.cost_of(MOVE));
        assert_eq!(settings.attack_cost, settings.cost_of(ATTACK));
        assert_eq!(settings.reproduce_cost, settings.cost_of(REPRODUCE));
        assert_eq!(settings.defile_cost, settings.cost_of(DEFILE));
        assert_eq!(0, settings.cost_of(PHOTOSYNTHESIS));
    }

    #[test]
    fn zero_has_no_costs() {
        let settings = Settings::zero();

        assert_eq!(0, settings.cost_of(SENSE));
        assert_eq!(0, settings.cost_of(TURN));
        assert_eq!(0, settings.cost_of(MOVE));
        assert_eq!(0, settings.cost_of(ATTACK));
        assert_eq!(0, settings.cost_of(REPRODUCE));
        assert_eq!(0, settings.cost_of(DEFILE));
    }
}
