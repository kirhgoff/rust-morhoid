use morphoid::types::*;
use std::slice::Iter;

impl Direction {
    pub const SIZE: usize = 8;

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

    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction;  Direction::SIZE] = [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ];
        DIRECTIONS.into_iter()
    }

    pub fn rotate(&self, value: Gene) -> Direction {
        let shift = (value % Direction::SIZE as u32) as usize;
        *Direction::iterator().skip(shift).next().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let north = Direction::North;
        assert_eq!(Direction::NorthEast, north.rotate(1));
        assert_eq!(Direction::South, north.rotate(12));
    }
}