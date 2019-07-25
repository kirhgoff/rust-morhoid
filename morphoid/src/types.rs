use std::collections::HashMap;

pub type Coords = i32;
pub type GenomeId = u64;
pub type Gene = usize;
pub type HealthType = i32;
pub type GeneIndex = usize; // TODO: rename in other places

pub const GENOME_LENGTH: usize = 64;
pub const GENE_COUNT: usize = 64;

// TODO: convert it to enums
pub const DEFILE: Gene = 25;
pub const SENSE: Gene = 26;
// Complex gene
pub const TURN: Gene = 27;
// Complex gene
pub const MOVE: Gene = 28;
pub const ATTACK: Gene = 29;
pub const REPRODUCE: Gene = 30;
pub const PHOTOSYNTHESIS: Gene = 31;

pub const KNOWN_GENES: [Gene; 7] = [
    DEFILE,
    SENSE,
    TURN,
    MOVE,
    ATTACK,
    REPRODUCE,
    PHOTOSYNTHESIS
];

pub struct SettingsBuilder {
    pub settings: Settings
}

#[derive(Debug, Clone)]
pub struct Settings {
    pub steps_per_turn: usize,
    pub reproduce_cost: HealthType,
    pub reproduce_threshold: HealthType, // TODO: not used
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(GenomeId),
    Corpse(HealthType),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    North = 0,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug)]
pub struct CellState {
    pub health: HealthType,
    pub direction: Direction,
}

pub struct CellStateStorage {
    pub states: HashMap<GenomeId, CellState>
}

pub struct Genome {
    pub id: GenomeId,
    pub genes: [Gene; GENOME_LENGTH],
}

pub struct GenomeState {
    pub current_gene: GeneIndex,
}

pub struct GenomeDesc {
    pub reproduces: usize,
    pub attacks: usize,
    pub photosynthesis: usize,
    pub defiles: usize
}

// TODO: move to processor
pub struct GenomeStorage {
    pub genomes: HashMap<GenomeId, Genome>,
    pub descriptors: HashMap<GenomeId, GenomeDesc>,
}

pub struct Processor {
    pub genome_states: HashMap<GenomeId, GenomeState>
}

pub struct World {
    pub width: Coords,
    pub height: Coords,
    pub settings: Settings,
    pub entities: Vec<Entity>,
    pub genomes: GenomeStorage,
    // TODO: move to processor
    pub cell_states: CellStateStorage,
}

pub trait Affector {
    fn set_cell(&mut self, x: Coords, y: Coords, genome: Genome);
    fn set_cell_ext(&mut self, x: Coords, y: Coords, genome: Genome, direction: Direction);
    fn set_nothing(&mut self, x: Coords, y: Coords);
    fn set_corpse(&mut self, x: Coords, y: Coords, value: HealthType);
    fn set_entity(&mut self, x: Coords, y: Coords, entity: Entity, genome: Option<Genome>, initial_state: Option<CellState>);

    fn move_cell(&mut self, x: Coords, y: Coords);
    fn rotate_cell(&mut self, x: Coords, y: Coords, value: Gene);

    fn punish_for_action(&mut self, x: Coords, y: Coords, gene: Gene);
    fn update_health(&mut self, x: Coords, y: Coords, health_delta: HealthType) -> HealthType;
    fn attack(&mut self, x: Coords, y: Coords, damage: HealthType);
    fn defile(&mut self, x: Coords, y: Coords, damage: HealthType);
    fn reproduce(&mut self, x: Coords, y: Coords);
    fn decay(&mut self, x: Coords, y: Coords, decay: HealthType);

    fn build_child_genome_for(&self, parent_genome_id: GenomeId) -> Option<Genome>;
}

pub trait Perceptor {
    fn get_entity(&self, x: Coords, y: Coords) -> &Entity;
    fn get_state(&self, genome_id: GenomeId) -> &CellState;
    fn get_state_by_pos(&self, x: Coords, y: Coords) -> Option<&CellState>;
    fn get_genome(&self, genome_id: GenomeId) -> Option<&Genome>;
    fn looking_at(&self, x: Coords, y: Coords) -> Option<(Coords, Coords)>;
}

pub trait Action {
    // do something with stats or replace with dirt
    fn execute(&self, affector: &mut Affector);
}

// TODO: make coords keep two coords
pub struct KillAction {
    pub x: Coords,
    pub y: Coords,
}

pub struct UpdateHealthAction {
    pub x: Coords,
    pub y: Coords,
    pub health_delta: HealthType,
}

pub struct ReproduceAction {
    pub x: Coords,
    pub y: Coords,
}

pub struct AttackAction {
    pub x: Coords,
    pub y: Coords,
    pub damage: HealthType,
}

pub struct MoveAction {
    pub x: Coords,
    pub y: Coords,
}

pub struct RotateAction {
    pub x: Coords,
    pub y: Coords,
    pub value: Gene, // new_direction += value % 8 ?
}

pub struct SenseAction {
    pub x: Coords,
    pub y: Coords,
    pub jump_nothing: Gene,
    pub jump_relative: Gene,
    pub jump_cell: Gene, // TODO: add enemy vs relative
}

pub struct DefileAction {
    pub x: Coords,
    pub y: Coords,
    pub damage: HealthType,
}

pub struct DecayAction {
    pub x: Coords,
    pub y: Coords,
    pub decay: HealthType,
}
