use crate::morphoid::types::*;

impl CellState {
    pub fn new(initial_health: HealthType, direction: Direction) -> CellState {
        CellState { health: initial_health, direction }
    }

    pub fn default() -> CellState {
        CellState { health: 10, direction: Direction::North }
    }
}