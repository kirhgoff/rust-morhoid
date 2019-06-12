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
    pub fn from<P : Projection>(world: &World, _: &P) -> WorldInfo {
        WorldInfo {
            width: world.width,
            height: world.height,
            data: format!("{}", world),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldInfo2 {
    pub width: Coords,
    pub height: Coords,
    pub data: Vec<Vec<String>>
}

impl WorldInfo2 {
    pub fn from<P : Projection>(world: &World, projection: &P) -> WorldInfo2 {
        let entities_info = world
            .entities
            .iter()
            .map(|entity| projection.from(entity, &world))
            .collect();

        WorldInfo2 {
            width: world.width,
            height: world.height,
            data: entities_info,
        }
    }
}


// Projection actually could return the whole entity info object
// which could be anything or part of enum
pub trait Projection {
    fn from(&self, entity: &Entity, world: &World) -> Vec<String>;
}

pub struct GeneTypesProjection;

// TODO: make it serialize GenomeDesc, this is just to make it work
impl Projection for GeneTypesProjection {
    fn from(&self, entity: &Entity, world: &World) -> Vec<String> {
        fn icon_for(desc: &GenomeDesc) -> char {
            match desc {
                // TODO: move all to ui, this is presentation layer, send whole desc
                x if x.reproduces > x.attacks + x.photosynthesys => '*',
                x if x.attacks > x.photosynthesys + x.defiles  => 'x',
                x if x.photosynthesys > x.attacks + x.defiles => 'o',
                x if x.defiles > x.attacks + x.photosynthesys => '@',
                _ => '.'
            }
        }

        let symbol = match entity {
            Entity::Nothing => ' ',
            Entity::Cell(genome_id) => icon_for(&world.genomes.describe(*genome_id).unwrap()),
            Entity::Corpse(_) => '+',
        };
        vec![symbol.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_info() {
        let mut world = World::prod(3, 1);
        world.set_cell(0, 0, Genome::new_plant());
        world.set_corpse(1, 0, 10);
        world.set_nothing(2, 0);

        let projection = GeneTypesProjection {};

        let world_info = WorldInfo2::from(&world, &projection);
        assert_eq!(world_info.width, 3);
        assert_eq!(world_info.height, 1);
        assert_eq!(world_info.data[0], vec!['o'.to_string()]);
        assert_eq!(world_info.data[1], vec!['+'.to_string()]);
        assert_eq!(world_info.data[2], vec![' '.to_string()]);
    }
}