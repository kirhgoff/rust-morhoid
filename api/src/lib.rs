#[cfg(feature = "server")]
pub mod methods;
pub mod types;

use morphoid::Coords;

pub const WORLD_WIDTH: Coords = 40;
pub const WORLD_HEIGHT: Coords = 40;
