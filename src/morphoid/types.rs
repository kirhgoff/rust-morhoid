use std::collections::HashMap;

pub type Coords = i32;
pub type GenomeId = u64;
pub type Gene = usize;
pub type HealthType = i32;
pub type GeneIndex = usize; // TODO: rename in other places

// TODO: rename to GENOME_LENGTH
pub const GENE_LENGTH: usize = 64;

pub const SENSE: Gene = 26; // Complex gene
pub const TURN: Gene = 27; // Complex gene
pub const MOVE: Gene = 28;
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
    pub attack_cost: HealthType,
    pub move_cost: HealthType,
    pub turn_cost: HealthType,
    pub sense_cost: HealthType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(GenomeId),
    Corpse(i64)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

pub struct CellState {
    pub health: HealthType,
    pub direction: Direction
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
    fn set_nothing(&mut self, x: Coords, y: Coords);
    fn set_entity(&mut self, x: Coords, y: Coords, entity: Entity, genome: Option<Genome>, initial_state: Option<CellState>);

    fn move_cell(&mut self, x: Coords, y: Coords);
    fn rotate_cell(&mut self, x: Coords, y: Coords, value: Gene);

    fn update_health(&mut self, x: Coords, y: Coords, health_delta: HealthType) -> HealthType;
    fn build_child_genome_for(&mut self, parent_genome_id: GenomeId) -> Option<Genome>;
    fn attack(&mut self, x: Coords, y: Coords, damage: HealthType);
}

pub trait Perceptor {
    // TODO do I need this method?
    fn get_state_mut(&mut self, hash: GenomeId) -> &mut CellState;

    fn get_entity(&self, x: Coords, y: Coords) -> &Entity;
    fn get_state(&self, hash: GenomeId) -> &CellState;
    fn get_genome(&self, hash: GenomeId) -> Option<&Genome>;
    fn looking_at(&self, x: Coords, y: Coords, hash: GenomeId) -> (Coords, Coords); // TODO: return type?
    fn find_vacant_place_around(&self, x:Coords, y:Coords) -> Option<(Coords, Coords)>;
    fn find_target_around(&self, x: Coords, y: Coords) -> Option<(Coords, Coords)>;
}

pub trait Action {
    // do something with stats or replace with dirt
    fn execute(&self, affector: &mut Affector);
}

// TODO: make coords keep two coords
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
    pub x: Coords,
    pub y: Coords,
    pub damage: HealthType
}

pub struct MoveAction {
    pub x: Coords,
    pub y: Coords,
}

pub struct RotateAction {
    pub x: Coords,
    pub y: Coords,
    pub value: Gene // new_direction += value % 8 ?
}

pub struct SenseAction {
    pub x: Coords,
    pub y: Coords,
    pub jump_nothing: Gene,
    pub jump_relative: Gene,
    pub jump_cell: Gene, // TODO: add enemy vs relative
}
