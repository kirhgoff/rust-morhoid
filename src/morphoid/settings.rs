use morphoid::types::Settings;
use morphoid::types::SettingsBuilder;
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
}

impl SettingsBuilder {
    pub fn prod() -> SettingsBuilder {
        SettingsBuilder {
            settings: Settings::prod()
        }
    }

    pub fn zero() -> Settings {
        SettingsBuilder::prod()
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

    pub fn with_steps_per_turn(&mut self, value: usize) -> &mut SettingsBuilder {
        self.settings.steps_per_turn = value; self
    }

    pub fn with_attack_damage(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.attack_damage = value; self
    }

    pub fn with_attack_cost(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.attack_cost = value; self
    }

    pub fn with_reproduce_cost(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.reproduce_cost = value; self
    }

    pub fn with_reproduce_threshold(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.reproduce_threshold = value; self
    }

    pub fn with_photosynthesys_adds(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.photosynthesys_adds = value; self
    }

    pub fn with_initial_cell_health(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.initial_cell_health = value; self
    }

    pub fn with_move_cost(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.move_cost = value; self
    }

    pub fn with_turn_cost(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.turn_cost = value; self
    }

    pub fn with_sense_cost(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.sense_cost = value; self
    }

    pub fn build(&mut self) -> Settings {
        self.settings
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morphoid::types::World;

    #[test]
    fn can_build2() {
        let settings = SettingsBuilder::prod().with_steps_per_turn(666).build();
        assert_eq!(settings.steps_per_turn(), 666);

        let _world = World::new(1, 1, settings);
    }
}
