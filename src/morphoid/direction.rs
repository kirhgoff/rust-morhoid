extern crate enum_primitive;
extern crate num;

use morphoid::types::*;
use std::slice::Iter;

impl Direction {
    pub const SIZE: usize = 8;

    pub fn shift(&self) -> (Coords, Coords) {
        use Direction::*;
        match *self {
            North => (0, -1),
            NorthEast => (1, -1),
            East => (1, 0),
            SouthEast => (1, 1),
            South => (0, 1),
            SouthWest => (1, -1),
            West => (-1, 0),
            NorthWest => (-1, -1),
        }
    }

    pub fn iterator() -> Iter<'static, Direction> {
        use Direction::*;
        static DIRECTIONS: [Direction;  Direction::SIZE] = [
            North,
            NorthEast,
            East,
            SouthEast,
            South,
            SouthWest,
            West,
            NorthWest,
        ];
        DIRECTIONS.into_iter()
    }

    /// value is Gene because it is passed from genome
    pub fn rotate(&self, value: Gene) -> Direction {
        use Direction::*;
        match Direction::from_i32(value as i32) {
            Some(North) => NorthEast,
            Some(NorthEast) => East,
            Some(East) => SouthEast,
            Some(SouthEast) => South,
            Some(South) => SouthWest,
            Some(SouthWest) => West,
            Some(West) => NorthWest,
            Some(NorthWest) => North,
            None => panic!(format!("ERROR: unexpected Direction value: {:?}", value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        assert_eq!(Direction::NorthEast, Direction::North.rotate(1));
        assert_eq!(Direction::East, Direction::NorthEast.rotate(1).rotate(1));

        assert_eq!(Direction::South, Direction::North.rotate(12));
    }
}