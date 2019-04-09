use morphoid::types::Settings;
use morphoid::types::HealthType;

impl Settings {
    // TODO: add builder

    pub fn prod() -> Settings {
        Settings {
            steps_per_turn: 1,
            reproduce_cost: -10,
            reproduce_threshold: 100,
            photosynthesys_adds: 1,
            initial_cell_health: 10,
            attack_damage: 4,
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