use morphoid::types::Settings;
use morphoid::types::HealthType;

impl Settings {
    // TODO: add builder

    pub fn prod() -> Settings {
        Settings {
            steps_per_turn: 1,
            reproduce_cost: -10,
            reproduce_threshold: 20,
            photosynthesys_adds: 5,
            initial_cell_health: 10,
            attack_damage: 100,
            attack_cost: -5,
            move_cost: -5,
            turn_cost: -5,
            sense_cost: -5
        }
    }

    pub fn steps_per_turn(&self) -> usize { self.steps_per_turn }
    pub fn reproduce_cost(&self) -> HealthType { self.reproduce_cost }
    pub fn reproduce_threshold(&self) -> HealthType { self.reproduce_threshold }
    pub fn photosynthesys_adds(&self) -> HealthType { self.photosynthesys_adds }
    pub fn initial_cell_health(&self) -> HealthType { self.initial_cell_health }
    pub fn attack_damage(&self) -> HealthType { self.attack_damage }
    pub fn attack_cost(&self) -> HealthType { self.attack_cost }
    pub fn move_cost(&self) -> HealthType { self.move_cost }
    pub fn turn_cost(&self) -> HealthType { self.turn_cost }
    pub fn sense_cost(&self) -> HealthType { self.sense_cost }

    pub fn with_steps_per_turn(&mut self, value: usize) -> &mut Settings {
        self.steps_per_turn = value; self
    }

    // TODO: make macros for that
    pub fn with_reproduce_cost(&mut self, value: HealthType) -> &mut Settings {
        self.reproduce_cost = value; self
    }

    pub fn with_reproduce_threshold(&mut self, value: HealthType) -> &mut Settings {
        self.reproduce_threshold = value; self
    }

    pub fn with_photosynthesys_adds(&mut self, value: HealthType) -> &mut Settings {
        self.photosynthesys_adds = value; self
    }

    pub fn with_initial_cell_health(&mut self, value: HealthType) -> &mut Settings {
        self.initial_cell_health = value; self
    }

    pub fn with_attack_damage(&mut self, value: HealthType) -> &mut Settings {
        self.attack_damage = value; self
    }

    pub fn with_attack_cost(&mut self, value: HealthType) -> &mut Settings {
        self.attack_cost = value; self
    }

    pub fn with_move_cost(&mut self, value: HealthType) -> &mut Settings {
        self.move_cost = value; self
    }

    pub fn with_turn_cost(&mut self, value: HealthType) -> &mut Settings {
        self.turn_cost = value; self
    }

    pub fn with_sense_cost(&mut self, value: HealthType) -> &mut Settings {
        self.sense_cost = value; self
    }

    pub fn build(&mut self) -> &Settings {
        self
    }

    pub fn zero() -> Settings {
        *Settings::prod()
        .with_reproduce_cost(0)
        .with_reproduce_threshold(0)
        .with_photosynthesys_adds(0)
        .with_attack_damage(0)
        .with_attack_cost(0)
        .with_move_cost(0)
        .with_turn_cost(0)
        .with_sense_cost(0)
        .build()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build() {
        let settings = Settings::prod().with_steps_per_turn(666);
        assert_eq!(settings.steps_per_turn(), 666);
    }
}
