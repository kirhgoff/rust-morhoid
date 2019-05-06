use morphoid::types::*;

impl Direction {
    pub const SIZE: usize = 8;

    const DIRECTIONS: [Direction;  Direction::SIZE] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn shift(&self) -> (Coords, Coords) {
        use Direction::*;
        match *self {
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (-1, 1),
            West => (-1, 0),
            NorthWest => (-1, -1),
        }
    }

    /// value is Gene because it is passed from genome
    pub fn rotate(&self, value: Gene) -> Direction {
        Direction::DIRECTIONS[(*self as usize + value) % Direction::SIZE]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(Direction::NorthEast, Direction::North.rotate(1));
        assert_eq!(Direction::East, Direction::North.rotate(2));
        assert_eq!(Direction::SouthEast, Direction::NorthEast.rotate(1).rotate(1));
        assert_eq!(Direction::South, Direction::North.rotate(12));
    }
}