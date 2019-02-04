use morphoid::types::*;

impl Direction {
    pub fn shift(&self) -> (Coords, Coords) {
        match *self {
            Direction::North => (-1, 0),
            Direction::NorthEast => (-1, 1),
            Direction::East => (0, -1),
            Direction::SouthEast => (1, 1),
            Direction::South => (1, 0),
            Direction::SouthWest => (1, -1),
            Direction::West => (0, -1),
            Direction::NorthWest => (-1, -1),
        }
    }
}