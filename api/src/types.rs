use serde::{Deserialize, Serialize};

use morphoid::types::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldInfo {
    pub width: Coords,
    pub height: Coords,
    // TODO: how to have any struct here
    pub data: Vec<Vec<String>>,
    pub meta: Vec<ProjectionRowMeta>
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
pub struct ProjectionRowMeta {
    name: String,
    comment: String,
    required: bool
}

impl ProjectionRowMeta {
    pub fn new(name: &str, comment: &str, required: bool) -> ProjectionRowMeta {
        ProjectionRowMeta {
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
    fn meta(&self) -> Vec<ProjectionRowMeta>;
}

pub struct GeneTypesProjection;
impl Projection for GeneTypesProjection {
    fn meta(&self) -> Vec<ProjectionRowMeta> {
        // TODO: make constant
        vec![
            ProjectionRowMeta::new("type", "Type of cell", true),
            ProjectionRowMeta::new("reproduces", "Number of reproducing genes", false),
            ProjectionRowMeta::new("attacks", "Number of attacking genes", false),
            ProjectionRowMeta::new("photosynthesis", "Number of genes, using solr power", false),
            ProjectionRowMeta::new("defiles", "Number of defiling genes", false),
            ProjectionRowMeta::new("health", "Current cell health", false)
        ]
    }

    fn from(&self, entity: &Entity, world: &World) -> Vec<String> {
        match entity {
            Entity::Nothing => vec![String::from("nothing")],
            Entity::Cell(genome_id) => {
                let state = world.get_state(*genome_id);
                let desc = world.genomes.describe(*genome_id).unwrap();
                vec![
                    String::from("cell"),
                    desc.reproduces.to_string(),
                    desc.attacks.to_string(),
                    desc.photosynthesis.to_string(),
                    desc.defiles.to_string(),
                    state.health.to_string()
                ]
            },
            Entity::Corpse(_) => vec![String::from("corpse")]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsInfo {
    pub reproduce_cost: HealthType,
    //pub reproduce_threshold: HealthType,
    pub photosynthesis_adds: HealthType,
    pub initial_cell_health: HealthType,
    pub attack_damage: HealthType,
    pub defile_damage: HealthType,
    pub attack_cost: HealthType,
    pub move_cost: HealthType,
    pub turn_cost: HealthType,
    pub sense_cost: HealthType,
    pub defile_cost: HealthType,
    pub corpse_decay: HealthType,
    pub corpse_initial: HealthType,
    pub mutation_probability: f64,
}

impl SettingsInfo {
    pub fn from(settings: &Settings) -> SettingsInfo {
        SettingsInfo {
            reproduce_cost: settings.reproduce_cost,
            //reproduce_threshold: settings.reproduce_threshold,
            photosynthesis_adds: settings.photosynthesis_adds,
            initial_cell_health: settings.initial_cell_health,
            attack_damage: settings.attack_damage,
            defile_damage: settings.defile_damage,
            attack_cost: settings.attack_cost,
            move_cost: settings.move_cost,
            turn_cost: settings.turn_cost,
            sense_cost: settings.sense_cost,
            defile_cost: settings.defile_cost,
            corpse_decay: settings.corpse_decay,
            corpse_initial: settings.corpse_initial,
            mutation_probability: settings.mutation_probability,
        }
    }

    pub fn as_settings(&self) -> Settings {
        SettingsBuilder::prod()
            .with_reproduce_cost(self.reproduce_cost)
            //.with_reproduce_threshold(self.reproduce_threshold)
            .with_photosynthesis_adds(self.photosynthesis_adds)
            .with_initial_cell_health(self.initial_cell_health)
            .with_attack_damage(self.attack_damage)
            .with_defile_damage(self.defile_damage)
            .with_attack_cost(self.attack_cost)
            .with_move_cost(self.move_cost)
            .with_turn_cost(self.turn_cost)
            .with_sense_cost(self.sense_cost)
            .with_defile_cost(self.defile_cost)
            .with_corpse_decay(self.corpse_decay)
            .with_corpse_initial(self.corpse_initial)
            .with_mutation_probability(self.mutation_probability)
            .build()
    }
}

// TODO: need better name
#[derive(Debug, Serialize, Deserialize)]
pub struct CellInfo {
    pub x: i32,
    pub y: i32,
    pub health: i32,
    pub direction: usize,
    pub genome_id: u64,
    pub genome: Vec<usize>
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
