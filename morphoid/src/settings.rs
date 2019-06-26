use crate::types::*;

impl Settings {
    // TODO: add builder

    pub fn prod() -> Settings {
        Settings {
            steps_per_turn: 1,
            reproduce_cost: -10,
            reproduce_threshold: 20,
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
            corpse_initial: 20
        }
    }

    pub fn steps_per_turn(&self) -> usize { self.steps_per_turn }
    pub fn reproduce_cost(&self) -> HealthType { self.reproduce_cost }
    pub fn reproduce_threshold(&self) -> HealthType { self.reproduce_threshold }
    pub fn photosynthesis_adds(&self) -> HealthType { self.photosynthesis_adds }
    pub fn initial_cell_health(&self) -> HealthType { self.initial_cell_health }
    pub fn attack_damage(&self) -> HealthType { self.attack_damage }
    pub fn attack_cost(&self) -> HealthType { self.attack_cost }
    pub fn move_cost(&self) -> HealthType { self.move_cost }
    pub fn turn_cost(&self) -> HealthType { self.turn_cost }
    pub fn sense_cost(&self) -> HealthType { self.sense_cost }
    pub fn defile_cost(&self) -> HealthType { self.defile_cost }
    pub fn defile_damage(&self) -> HealthType { self.defile_damage }
    pub fn corpse_decay(&self) -> HealthType { self.corpse_decay }
    pub fn corpse_initial(&self) -> HealthType { self.corpse_initial }
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
            .with_photosynthesis_adds(0)
            .with_attack_damage(0)
            .with_attack_cost(0)
            .with_move_cost(0)
            .with_turn_cost(0)
            .with_sense_cost(0)
            .with_defile_cost(0)
            .with_defile_damage(0)
            .with_corpse_decay(0)
            .with_corpse_initial(0)
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

    pub fn with_photosynthesis_adds(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.photosynthesis_adds = value; self
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

    pub fn with_defile_cost(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.defile_cost = value; self
    }

    pub fn with_defile_damage(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.defile_damage = value; self
    }

    pub fn with_corpse_decay(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.corpse_decay = value; self
    }

    pub fn with_corpse_initial(&mut self, value: HealthType) -> &mut SettingsBuilder {
        self.settings.corpse_initial = value; self
    }

    pub fn build(&mut self) -> Settings {
        self.settings.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder() {
        let settings = SettingsBuilder::prod()
            .with_reproduce_cost(1)
            .with_reproduce_threshold(2)
            .with_photosynthesis_adds(3)
            .with_attack_damage(4)
            .with_attack_cost(5)
            .with_move_cost(6)
            .with_turn_cost(7)
            .with_sense_cost(8)
            .with_defile_damage(9)
            .with_corpse_decay(10)
            .with_corpse_initial(11)
            .with_defile_cost(12)
            .build();

        assert_eq!(1, settings.reproduce_cost());
        assert_eq!(2, settings.reproduce_threshold());
        assert_eq!(3, settings.photosynthesis_adds());
        assert_eq!(4, settings.attack_damage());
        assert_eq!(5, settings.attack_cost());
        assert_eq!(6, settings.move_cost());
        assert_eq!(7, settings.turn_cost());
        assert_eq!(8, settings.sense_cost());
        assert_eq!(9, settings.defile_damage());
        assert_eq!(10, settings.corpse_decay());
        assert_eq!(11, settings.corpse_initial());
        assert_eq!(12, settings.defile_cost());
    }
}
