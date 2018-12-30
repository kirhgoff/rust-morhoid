use morphoid::cell_state::HealthType;

pub struct Settings {

    // TODO: add builder
    pub steps_per_turn: usize,
    pub reproduce_cost: HealthType,
    pub reproduce_threshold: HealthType,
    pub photosynthesys_adds: HealthType,
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            steps_per_turn: 15,
            reproduce_cost: -10,
            reproduce_threshold: 100,
            photosynthesys_adds: 1
        }
    }

    pub fn steps_per_turn(&self) -> usize { self.steps_per_turn }
    pub fn reproduce_cost(&self) -> HealthType { self.reproduce_cost }
    pub fn reproduce_threshold(&self) -> HealthType { self.reproduce_threshold }
    pub fn photosynthesys_adds(&self) -> HealthType { self.photosynthesys_adds }
}