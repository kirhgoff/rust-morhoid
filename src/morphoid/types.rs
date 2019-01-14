use std::collections::HashMap;

pub type Coords = i32;
pub type GenomeId = u64;
pub type Gene = u32;
pub type HealthType = i32;
pub type GeneIndex = usize; // TODO: rename in other places

// TODO: rename to GENOME_LENGTH
pub const GENE_LENGTH: GeneIndex = 64;

pub const ATTACK: Gene = 29;
pub const REPRODUCE: Gene = 30;
pub const PHOTOSYNTHESYS: Gene = 31;

pub struct Settings {
    pub steps_per_turn: usize,
    pub reproduce_cost: HealthType,
    pub reproduce_threshold: HealthType,
    pub photosynthesys_adds: HealthType,
    pub initial_cell_health: HealthType,
    pub attack_damage: HealthType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(GenomeId),
    Corpse(i64)
}

pub struct CellState {
    pub health: HealthType
}

pub struct CellStateStorage {
    pub states: HashMap<GenomeId,CellState>
}

pub struct Genome {
    pub id: GenomeId,
    pub genes: [Gene; GENE_LENGTH]
}

pub struct GenomeState {
    pub current_gene: GeneIndex,
}

// TODO: move to processor
pub struct GenomeStorage {
    pub genomes: HashMap<GenomeId, Genome>
}

pub struct Processor {
    pub genome_states: HashMap<GenomeId, GenomeState>
}

pub struct World {
    pub width: Coords,
    pub height: Coords,
    pub settings: Settings,
    pub entities: Vec<Entity>,
    pub genomes: GenomeStorage, // TODO: move to processor
    pub cell_states: CellStateStorage,
}

pub trait Affector {
    fn set_cell(&mut self, x:Coords, y:Coords, genome:Genome);
    fn set_nothing(&mut self, x:Coords, y:Coords);
    fn set_entity(&mut self, x:Coords, y:Coords, entity: Entity, genome: Option<Genome>, initial_state: Option<CellState>);

    fn update_health(&mut self, x:Coords, y:Coords, health_delta: HealthType);
    fn build_child_genome_for(&mut self, parent_genome_id: GenomeId) -> Some(Genome); //TODO... continue
}

pub trait Perceptor {
    // TODO do I need this method?
    fn get_state_mut(&mut self, hash: GenomeId) -> &mut CellState;

    fn get_entity(&self, x:Coords, y:Coords) -> &Entity;
    fn get_state(&self, hash: GenomeId) -> &CellState;
    fn get_genome(&self, hash: GenomeId) -> &Genome;
    fn find_vacant_place_around(&self, x:Coords, y:Coords) -> Option<(Coords, Coords)>;
    fn find_target_around(&self, x: Coords, y: Coords) -> Option<(Coords, Coords)>;
}

pub trait Action {
    // do something with stats or replace with dirt
    fn execute(&self, affector: &mut Affector);
}

pub struct KillAction {
    pub x: Coords,
    pub y: Coords
}

pub struct UpdateHealthAction {
    pub x: Coords,
    pub y: Coords,
    pub health_delta: HealthType
}

pub struct ReproduceAction {
    pub x: Coords,
    pub y: Coords,
    pub parent_genome_id: GenomeId
}

pub struct AttackAction {
    pub victim_x: Coords,
    pub victim_y: Coords,
    pub attacker_x: Coords,
    pub attacker_y: Coords,
    pub damage: HealthType
}

