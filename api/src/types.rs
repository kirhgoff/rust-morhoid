use serde::{Deserialize, Serialize};

use morphoid::{Coords, Entity, HealthType, Settings, World};

#[derive(Debug, Serialize, Deserialize)]
pub struct WorldInfo {
    pub width: Coords,
    pub height: Coords,
    pub data: Vec<Vec<String>>,
    pub meta: Vec<ProjectionRowMeta>,
}

impl From<&World> for WorldInfo {
    fn from(world: &World) -> WorldInfo {
        WorldInfo {
            width: world.width,
            height: world.height,
            data: world
                .entities
                .iter()
                .map(|entity| entity_row(entity, world))
                .collect(),
            meta: row_meta(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectionRowMeta {
    name: String,
    comment: String,
    required: bool,
}

impl ProjectionRowMeta {
    pub fn new(name: &str, comment: &str, required: bool) -> ProjectionRowMeta {
        ProjectionRowMeta {
            name: name.into(),
            comment: comment.into(),
            required,
        }
    }
}

fn row_meta() -> Vec<ProjectionRowMeta> {
    vec![
        ProjectionRowMeta::new("type", "Type of cell", true),
        ProjectionRowMeta::new("reproduces", "Number of reproducing genes", false),
        ProjectionRowMeta::new("attacks", "Number of attacking genes", false),
        ProjectionRowMeta::new("photosynthesis", "Number of genes, using solr power", false),
        ProjectionRowMeta::new("defiles", "Number of defiling genes", false),
        ProjectionRowMeta::new("health", "Current cell health", false),
    ]
}

fn entity_row(entity: &Entity, world: &World) -> Vec<String> {
    match entity {
        Entity::Nothing => vec![String::from("nothing")],
        Entity::Cell(genome_id) => {
            let cell = world.cell(*genome_id);
            vec![
                String::from("cell"),
                cell.desc.reproduces.to_string(),
                cell.desc.attacks.to_string(),
                cell.desc.photosynthesis.to_string(),
                cell.desc.defiles.to_string(),
                cell.health.to_string(),
            ]
        }
        Entity::Corpse(_) => vec![String::from("corpse")],
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsInfo {
    pub reproduce_cost: HealthType,
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

impl From<&Settings> for SettingsInfo {
    fn from(settings: &Settings) -> SettingsInfo {
        SettingsInfo {
            reproduce_cost: settings.reproduce_cost,
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
}

impl SettingsInfo {
    pub fn validate(&self) -> Result<(), &'static str> {
        if (0.0..=1.0).contains(&self.mutation_probability) {
            Ok(())
        } else {
            Err("mutation_probability must be within [0, 1]")
        }
    }
}

impl From<&SettingsInfo> for Settings {
    fn from(info: &SettingsInfo) -> Settings {
        Settings {
            reproduce_cost: info.reproduce_cost,
            photosynthesis_adds: info.photosynthesis_adds,
            initial_cell_health: info.initial_cell_health,
            attack_damage: info.attack_damage,
            defile_damage: info.defile_damage,
            attack_cost: info.attack_cost,
            move_cost: info.move_cost,
            turn_cost: info.turn_cost,
            sense_cost: info.sense_cost,
            defile_cost: info.defile_cost,
            corpse_decay: info.corpse_decay,
            corpse_initial: info.corpse_initial,
            mutation_probability: info.mutation_probability,
            ..Settings::prod()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CellInfo {
    pub x: i32,
    pub y: i32,
    pub health: i32,
    pub direction: usize,
    pub genome_id: u64,
    pub genome: Vec<usize>,
}

impl CellInfo {
    pub fn at(world: &World, x: Coords, y: Coords) -> Option<CellInfo> {
        let cell = world.cell_at(x, y)?;
        Some(CellInfo {
            x,
            y,
            health: cell.health,
            direction: cell.direction as usize,
            genome_id: cell.genome.id,
            genome: cell.genome.genes.to_vec(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morphoid::Genome;

    #[test]
    fn test_world_info() {
        let mut world = World::prod(3, 2);

        world.set_cell(0, 0, Genome::new_plant());
        world.set_corpse(1, 0, 10);
        world.set_nothing(2, 0);

        world.set_cell(0, 1, Genome::new_predator());
        world.set_cell(1, 1, Genome::new_yeast());
        world.set_cell(2, 1, Genome::new_defiler());

        let world_info = WorldInfo::from(&world);

        assert_eq!(world_info.width, 3);
        assert_eq!(world_info.height, 2);

        assert_eq!(
            world_info.data[0],
            fixture(vec!["cell", "0", "0", "64", "0", "10"])
        );
        assert_eq!(world_info.data[1], fixture(vec!["corpse"]));
        assert_eq!(world_info.data[2], fixture(vec!["nothing"]));

        assert_eq!(
            world_info.data[3],
            fixture(vec!["cell", "0", "64", "0", "0", "10"])
        );
        assert_eq!(
            world_info.data[4],
            fixture(vec!["cell", "64", "0", "0", "0", "10"])
        );
        assert_eq!(
            world_info.data[5],
            fixture(vec!["cell", "0", "0", "0", "64", "10"])
        );
    }

    fn fixture(source: Vec<&str>) -> Vec<String> {
        source.iter().map(|e| e.to_string()).collect()
    }
}
