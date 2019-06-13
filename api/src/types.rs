use serde::{Deserialize, Serialize};

use morphoid::types::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldInfo {
    pub width: Coords,
    pub height: Coords,
    // TODO: how to have any struct here
    pub data: Vec<Vec<String>>,
    pub meta: Vec<ProjectionRawMeta>
}

impl WorldInfo {
    pub fn from<P : Projection>(world: &World, projection: &P) -> WorldInfo {
        let entities_info = world
            .entities
            .iter()
            .map(|entity| projection.from(entity, &world))
            .collect();

        WorldInfo {
            width: world.width,
            height: world.height,
            data: entities_info,
            meta: projection.meta()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectionRawMeta {
    name: String,
    comment: String,
    required: bool
}

impl ProjectionRawMeta {
    pub fn new(name: &str, comment: &str, required: bool) -> ProjectionRawMeta {
        ProjectionRawMeta {
            name: name.into(),
            comment: comment.into(),
            required
        }
    }
}

// Projection actually could return the whole entity info object
// which could be anything or part of enum
pub trait Projection {
    fn from(&self, entity: &Entity, world: &World) -> Vec<String>;
    fn meta(&self) -> Vec<ProjectionRawMeta>;
}

pub struct GeneTypesProjection;
impl Projection for GeneTypesProjection {
    fn meta(&self) -> Vec<ProjectionRawMeta> {
        // TODO: make constant
        vec![
            ProjectionRawMeta::new("type", "Type of cell", true),
            ProjectionRawMeta::new("reproduces", "Number of reproducing genes", false),
            ProjectionRawMeta::new("attacks", "Number of attacking genes", false),
            ProjectionRawMeta::new("photosynthesys", "Number of genes, using solr power", false),
            ProjectionRawMeta::new("defiles", "Number of defiling genes", false)
        ]
    }

    fn from(&self, entity: &Entity, world: &World) -> Vec<String> {
        match entity {
            Entity::Nothing => vec![String::from("nothing")],
            Entity::Cell(genome_id) => {
                let desc = world.genomes.describe(*genome_id).unwrap();
                vec![
                    String::from("cell"),
                    desc.reproduces.to_string(),
                    desc.attacks.to_string(),
                    desc.photosynthesys.to_string(),
                    desc.defiles.to_string()
                ]
            },
            Entity::Corpse(_) => vec![String::from("corpse")]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_info() {
        let mut world = World::prod(3, 2);

        world.set_cell(0, 0, Genome::new_plant());
        world.set_corpse(1, 0, 10);
        world.set_nothing(2, 0);

        world.set_cell(0, 1, Genome::new_predator());
        world.set_cell(1, 1, Genome::new_yeast());
        world.set_cell(2, 1, Genome::new_defiler());

        let projection = GeneTypesProjection {};

        let world_info = WorldInfo::from(&world, &projection);

        assert_eq!(world_info.width, 3);
        assert_eq!(world_info.height, 2);

        assert_eq!(world_info.data[0], fixture(vec!["cell", "0", "0", "64", "0"]));
        assert_eq!(world_info.data[1], fixture(vec!["corpse"]));
        assert_eq!(world_info.data[2], fixture(vec!["nothing"]));

        assert_eq!(world_info.data[3], fixture(vec!["cell", "0", "64", "0", "0"]));
        assert_eq!(world_info.data[4], fixture(vec!["cell", "64", "0", "0", "0"]));
        assert_eq!(world_info.data[5], fixture(vec!["cell", "0", "0", "0", "64"]));
    }

    fn fixture(source: Vec<&str>) -> Vec<String> {
        source.iter().map(|e| e.to_string()).collect()
    }
}