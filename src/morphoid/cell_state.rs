use morphoid::types::*;

impl CellState {
    pub fn new(initial_health: HealthType, direction: Direction) -> CellState {
        CellState { health: initial_health, direction: direction }
    }

    pub fn new_from(settings: Settings) -> CellState {
        CellState { health: settings.initial_cell_health, direction: Direction::North }
    }

    pub fn default() -> CellState {
        CellState { health: 10, direction: Direction::North }
    }
}