use morphoid::types::Settings;
use morphoid::types::HealthType;

impl Settings {
    // TODO: add builder

    pub fn prod() -> Settings {
        Settings {
            steps_per_turn: 15,
            reproduce_cost: -10,
            reproduce_threshold: 100,
            photosynthesys_adds: 1,
            initial_cell_health: 10,
            attack_damage: 4,
        }
    }

    pub fn steps_per_turn(&self) -> usize { self.steps_per_turn }
    pub fn reproduce_cost(&self) -> HealthType { self.reproduce_cost }
    pub fn reproduce_threshold(&self) -> HealthType { self.reproduce_threshold }
    pub fn photosynthesys_adds(&self) -> HealthType { self.photosynthesys_adds }
    pub fn initial_cell_health(&self) -> HealthType { self.initial_cell_health }
    pub fn attack_damage(&self) -> HealthType { self.attack_damage }
}