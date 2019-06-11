extern crate serde;
use serde::{Deserialize, Serialize};

use crate::morphoid::types::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldInfo {
    pub width: Coords,
    pub height: Coords,
    pub data: String
}

impl WorldInfo {
    pub fn from(world: &World) -> WorldInfo {
        WorldInfo {
            width: world.width,
            height: world.height,
            data: format!("{}", world),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_info() {
        let world = World::prod(2, 3);
        let world_info = WorldInfo::from(&world);
        assert_eq!(2, world_info.width);
        assert_eq!(3, world_info.height);
    }
}