use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

use rand::Rng;

use crate::action::Action;
use crate::direction::Direction;
use crate::genome::{Gene, Genome, GenomeDesc, GenomeId, ATTACK, DEFILE, MOVE, REPRODUCE, TURN};
use crate::processor;
use crate::settings::Settings;

pub type Coords = i32;
pub type HealthType = i32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Entity {
    Nothing,
    Cell(GenomeId),
    Corpse(HealthType),
}

pub struct Cell {
    pub genome: Genome,
    pub desc: GenomeDesc,
    pub health: HealthType,
    pub direction: Direction,
    pub gene_index: usize,
}

pub struct World {
    pub width: Coords,
    pub height: Coords,
    pub settings: Settings,
    pub entities: Vec<Entity>,
    pub cells: HashMap<GenomeId, Cell>,
}

const CELL_DENSITY: (u32, u32) = (1, 3);

impl World {
    pub fn prod(width: Coords, height: Coords) -> World {
        World::new(width, height, Settings::prod())
    }

    pub fn random(width: Coords, height: Coords, settings: Settings) -> World {
        let mut rng = rand::thread_rng();
        let mut world = World::new(width, height, settings);

        for x in 0..width {
            for y in 0..height {
                if rng.gen_ratio(CELL_DENSITY.0, CELL_DENSITY.1) {
                    let genome = Genome::random(&mut rng);
                    let direction = Direction::random(&mut rng);
                    world.set_cell_facing(x, y, genome, direction);
                } else {
                    world.set_nothing(x, y);
                }
            }
        }
        world
    }

    pub fn new(width: Coords, height: Coords, settings: Settings) -> World {
        let entities = (0..width * height).map(|_| Entity::Nothing).collect();

        World {
            width,
            height,
            settings,
            entities,
            cells: HashMap::new(),
        }
    }

    pub fn tick(&mut self) {
        let mut actions: Vec<Action> = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                match self.entity_at(x, y) {
                    Entity::Cell(id) => {
                        let (mut planned, next_gene) = processor::run_genome(self, x, y, id);
                        actions.append(&mut planned);
                        self.cells.get_mut(&id).unwrap().gene_index = next_gene;
                    }
                    Entity::Corpse(_) => actions.push(Action::Decay {
                        x,
                        y,
                        amount: self.settings.corpse_decay,
                    }),
                    Entity::Nothing => {}
                }
            }
        }
        self.apply(&actions);
    }

    pub fn apply(&mut self, actions: &[Action]) {
        for action in actions {
            self.perform(*action)
        }
    }

    fn perform(&mut self, action: Action) {
        match action {
            Action::UpdateHealth { x, y, delta } => {
                self.update_health(x, y, delta);
            }
            Action::Reproduce { x, y } => {
                self.charge(x, y, REPRODUCE);
                self.reproduce(x, y)
            }
            Action::Attack { x, y, damage } => {
                self.charge(x, y, ATTACK);
                self.attack(x, y, damage)
            }
            Action::Move { x, y } => {
                self.charge(x, y, MOVE);
                self.move_cell(x, y)
            }
            Action::Rotate { x, y, by } => {
                self.charge(x, y, TURN);
                self.rotate_cell(x, y, by)
            }
            Action::Defile { x, y, damage } => {
                self.charge(x, y, DEFILE);
                self.defile(x, y, damage)
            }
            Action::Decay { x, y, amount } => self.decay(x, y, amount),
        }
    }

    fn get_index(&self, x: Coords, y: Coords) -> usize {
        let x2 = World::normalize(x, self.width);
        let y2 = World::normalize(y, self.height);

        (y2 * self.width + x2) as usize
    }

    fn normalize(coord: Coords, dimension: Coords) -> Coords {
        let remainder = coord % dimension;
        if coord < 0 {
            if remainder == 0 {
                0
            } else {
                dimension + remainder
            }
        } else {
            remainder
        }
    }

    pub fn entity_at(&self, x: Coords, y: Coords) -> Entity {
        self.entities[self.get_index(x, y)]
    }

    pub fn cell(&self, id: GenomeId) -> &Cell {
        &self.cells[&id]
    }

    pub fn cell_at(&self, x: Coords, y: Coords) -> Option<&Cell> {
        match self.entity_at(x, y) {
            Entity::Cell(id) => self.cells.get(&id),
            _ => None,
        }
    }

    fn cell_at_mut(&mut self, x: Coords, y: Coords) -> Option<&mut Cell> {
        match self.entity_at(x, y) {
            Entity::Cell(id) => self.cells.get_mut(&id),
            _ => None,
        }
    }

    pub fn looking_at(&self, x: Coords, y: Coords) -> Option<(Coords, Coords)> {
        let (dx, dy) = self.cell_at(x, y)?.direction.shift();
        Some((x + dx, y + dy))
    }

    pub fn set_nothing(&mut self, x: Coords, y: Coords) {
        self.replace(x, y, Entity::Nothing);
    }

    pub fn set_corpse(&mut self, x: Coords, y: Coords, remains: HealthType) {
        self.replace(x, y, Entity::Corpse(remains));
    }

    pub fn set_cell(&mut self, x: Coords, y: Coords, genome: Genome) {
        self.set_cell_facing(x, y, genome, Direction::North);
    }

    pub fn set_cell_facing(&mut self, x: Coords, y: Coords, genome: Genome, direction: Direction) {
        self.replace(x, y, Entity::Cell(genome.id));
        self.cells.insert(
            genome.id,
            Cell {
                desc: GenomeDesc::of(&genome),
                health: self.settings.initial_cell_health,
                direction,
                gene_index: 0,
                genome,
            },
        );
    }

    fn replace(&mut self, x: Coords, y: Coords, entity: Entity) {
        let index = self.get_index(x, y);
        if let Entity::Cell(old) = self.entities[index] {
            self.cells.remove(&old);
        }
        self.entities[index] = entity;
    }

    fn move_cell(&mut self, x: Coords, y: Coords) {
        let old_index = self.get_index(x, y);

        if let Entity::Cell(genome_id) = self.entities[old_index] {
            if let Some((new_x, new_y)) = self.looking_at(x, y) {
                let new_index = self.get_index(new_x, new_y);

                if self.entities[new_index] == Entity::Nothing {
                    self.entities[new_index] = Entity::Cell(genome_id);
                    self.entities[old_index] = Entity::Nothing;
                }
            }
        }
    }

    fn rotate_cell(&mut self, x: Coords, y: Coords, value: Gene) {
        if let Some(cell) = self.cell_at_mut(x, y) {
            cell.direction = cell.direction.rotate(value);
        }
    }

    fn update_health(&mut self, x: Coords, y: Coords, delta: HealthType) -> HealthType {
        let Some(cell) = self.cell_at_mut(x, y) else {
            return 0;
        };
        let old_health = cell.health;
        cell.health = cell.health.saturating_add(delta);

        if cell.health >= 0 {
            return -delta;
        }

        let corpse_health = self.settings.corpse_initial;
        self.set_corpse(x, y, corpse_health);
        old_health
    }

    fn defile(&mut self, x: Coords, y: Coords, damage: HealthType) {
        if let Entity::Cell(_) = self.entity_at(x, y) {
            if let Some((new_x, new_y)) = self.looking_at(x, y) {
                if let Entity::Corpse(remains) = self.entity_at(new_x, new_y) {
                    let mut result = damage;
                    let new_remains = remains - damage;
                    if new_remains > 0 {
                        self.set_corpse(new_x, new_y, new_remains);
                    } else {
                        self.set_nothing(new_x, new_y);
                        result = remains;
                    }
                    self.update_health(x, y, result);
                }
            }
        }
    }

    fn charge(&mut self, x: Coords, y: Coords, gene: Gene) {
        let cost = self.settings.cost_of(gene);
        self.update_health(x, y, cost);
    }

    fn attack(&mut self, x: Coords, y: Coords, damage: HealthType) {
        if let Entity::Cell(_) = self.entity_at(x, y) {
            if let Some((new_x, new_y)) = self.looking_at(x, y) {
                let health_eaten = self.update_health(new_x, new_y, -damage);
                self.update_health(x, y, health_eaten);
            }
        }
    }

    fn reproduce(&mut self, x: Coords, y: Coords) {
        let new_genome = self.cell_at(x, y).map(|cell| {
            cell.genome
                .offspring(self.settings.mutation_probability, &mut rand::thread_rng())
        });

        if let Some(new_genome) = new_genome {
            if let Some((new_x, new_y)) = self.looking_at(x, y) {
                if !matches!(self.entity_at(new_x, new_y), Entity::Cell(_)) {
                    let mut rng = rand::thread_rng();
                    let direction = Direction::random(&mut rng);
                    self.set_cell_facing(new_x, new_y, new_genome, direction);
                }
            }
        }
    }

    fn decay(&mut self, x: Coords, y: Coords, amount: HealthType) {
        let Entity::Corpse(remains) = self.entity_at(x, y) else {
            return;
        };
        let left = remains.saturating_add(amount);
        if left > 0 {
            self.set_corpse(x, y, left);
        } else {
            self.set_nothing(x, y);
        }
    }
}

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn icon_for(desc: &GenomeDesc) -> char {
            match desc {
                x if x.reproduces > x.attacks + x.photosynthesis => '*',
                x if x.attacks > x.photosynthesis + x.defiles => 'x',
                x if x.photosynthesis > x.attacks + x.defiles => 'o',
                x if x.defiles > x.attacks + x.photosynthesis => '@',
                _ => '.',
            }
        }

        for line in self.entities.as_slice().chunks(self.width as usize) {
            for &entity in line {
                let symbol = match entity {
                    Entity::Nothing => ' ',
                    Entity::Cell(genome_id) => icon_for(&self.cell(genome_id).desc),
                    Entity::Corpse(_) => '+',
                };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_index() {
        let world = World::prod(2, 1);

        assert_eq!(world.get_index(-2, 0), 0);
        assert_eq!(world.get_index(-1, 0), 1);
        assert_eq!(world.get_index(0, 0), 0);
        assert_eq!(world.get_index(1, 0), 1);
        assert_eq!(world.get_index(2, 0), 0);

        assert_eq!(world.get_index(0, 0), 0);
        assert_eq!(world.get_index(0, 1), 0);
        assert_eq!(world.get_index(0, 2), 0);
        assert_eq!(world.get_index(0, -2), 0);
        assert_eq!(world.get_index(0, -1), 0);

        assert_eq!(world.get_index(1, 0), 1);
        assert_eq!(world.get_index(1, 1), 1);
        assert_eq!(world.get_index(1, 2), 1);
        assert_eq!(world.get_index(1, -2), 1);
        assert_eq!(world.get_index(1, -1), 1);
    }

    #[test]
    fn test_looking_at() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();

        world.set_cell_facing(1, 0, plant, Direction::North);

        assert_eq!(Some((0, -1)), world.looking_at(0, 0));

        world.rotate_cell(0, 0, 1);
        assert_eq!(Some((1, -1)), world.looking_at(0, 0));

        world.rotate_cell(0, 0, 1);
        assert_eq!(Some((1, 0)), world.looking_at(0, 0));

        world.rotate_cell(0, 0, 1);
        assert_eq!(Some((1, 1)), world.looking_at(0, 0));

        world.rotate_cell(0, 0, 2);
        assert_eq!(Some((-1, 1)), world.looking_at(0, 0));
    }

    #[test]
    fn test_update_health_addition() {
        let settings = Settings::prod();
        let initial_cell_health = settings.initial_cell_health;

        let mut world = World::new(1, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());

        assert_eq!(world.update_health(0, 0, 10), -10);

        assert_eq!(
            world
                .cell_at(0, 0)
                .expect("There should be cell here")
                .health,
            initial_cell_health + 10
        );
    }

    #[test]
    fn test_update_health_small_damage() {
        let settings = Settings::prod();
        let initial_cell_health = settings.initial_cell_health;

        let mut world = World::new(1, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());

        assert_eq!(
            world.update_health(0, 0, -initial_cell_health + 5),
            initial_cell_health - 5
        );

        assert_eq!(
            world
                .cell_at(0, 0)
                .expect("There should be cell here")
                .health,
            5
        );
    }

    #[test]
    fn test_update_health_big_damage() {
        let settings = Settings::prod();
        let initial_cell_health = settings.initial_cell_health;

        let mut world = World::new(1, 1, settings);
        world.set_cell(0, 0, Genome::new_plant());

        assert_eq!(
            world.update_health(0, 0, -initial_cell_health - 1),
            initial_cell_health
        );

        assert!(
            !matches!(world.entity_at(0, 0), Entity::Cell(_)),
            "cell should be dead"
        );
    }

    #[test]
    fn random_world_has_cells() {
        let world = World::random(40, 40, Settings::prod());
        assert!(!world.cells.is_empty());
    }

    #[test]
    fn set_nothing_removes_cell() {
        let mut world = World::prod(1, 1);
        let plant = Genome::new_plant();
        let id = plant.id;

        world.set_cell(0, 0, plant);
        assert!(world.cells.contains_key(&id));

        world.set_nothing(0, 0);
        assert!(!world.cells.contains_key(&id));
        assert!(matches!(world.entity_at(0, 0), Entity::Nothing));
    }

    #[test]
    fn integration_test_it_reproduces() {
        let settings = Settings {
            photosynthesis_adds: 5,
            initial_cell_health: 10,
            reproduce_cost: -6,
            ..Settings::prod()
        };

        let initial_cell_health = settings.initial_cell_health;

        let new_value = settings.initial_cell_health + settings.reproduce_cost;

        let mut world = World::new(2, 1, settings);

        let plant = Genome::new_yeast();
        let genome_id = plant.id;

        world.set_nothing(0, 0);
        world.set_cell_facing(1, 0, plant, Direction::West);

        world.tick();

        let current_health = world.cell_at(1, 0).unwrap().health;

        assert_eq!(new_value, current_health);

        match world.entity_at(0, 0) {
            Entity::Cell(another) => assert_ne!(another, genome_id),
            _ => panic!("new cell was not reproduced"),
        }

        assert_eq!(world.cell_at(0, 0).unwrap().health, initial_cell_health);
    }

    #[test]
    fn integration_test_and_then_there_were_none() {
        let settings = Settings {
            attack_cost: 1,
            attack_damage: 100,
            corpse_decay: 0,
            ..Settings::prod()
        };
        let mut world = World::new(3, 3, settings);

        for x in 0..3 {
            for y in 0..3 {
                if x != 1 || y != 1 {
                    world.set_cell(x, y, Genome::new_plant());
                }
            }
        }

        let mut genome = Genome::new_predator();
        for i in (0..27).step_by(3) {
            genome.mutate(i, ATTACK);
            genome.mutate(i + 1, TURN);
            genome.mutate(i + 2, 1);
        }

        world.set_cell_facing(1, 1, genome, Direction::North);

        for _ in 0..15 {
            world.tick()
        }

        for x in 0..3 {
            for y in 0..3 {
                if (x, y) != (1, 1) {
                    assert!(
                        matches!(world.entity_at(x, y), Entity::Corpse(_)),
                        "victim at ({x}, {y}) survived"
                    );
                }
            }
        }

        assert!(
            matches!(world.entity_at(1, 1), Entity::Cell(_)),
            "the killer must survive"
        );
    }

    #[test]
    fn integration_test_plant_reproduce_if_have_enough() {
        let settings = Settings {
            reproduce_cost: 0,
            ..Settings::prod()
        };

        let mut world = World::new(2, 1, settings);
        let mut plant = Genome::new_plant();
        plant.mutate(1, REPRODUCE);
        let hash = plant.id;

        world.set_cell_facing(0, 0, plant, Direction::East);
        world.set_nothing(1, 0);

        world.tick();

        assert!(
            matches!(world.entity_at(1, 0), Entity::Nothing),
            "new cell should not have been created yet"
        );

        world.tick();

        match world.entity_at(1, 0) {
            Entity::Cell(another_hash) => assert_ne!(another_hash, hash),
            _ => panic!("new cell was not reproduced"),
        }
    }

    #[test]
    fn integration_test_order_of_execution_parent_killed() {
        let mut world = World::new(3, 1, Settings::zero());

        world.set_cell_facing(1, 0, Genome::new_yeast(), Direction::West);
        world.set_cell_facing(2, 0, Genome::new_predator(), Direction::West);

        world.apply(&[
            Action::Attack {
                x: 2,
                y: 0,
                damage: 100,
            },
            Action::Reproduce { x: 1, y: 0 },
        ]);

        assert!(
            matches!(world.entity_at(1, 0), Entity::Corpse(_)),
            "parent should have been destroyed"
        );

        assert!(
            matches!(world.entity_at(0, 0), Entity::Nothing),
            "nothing should be born"
        );
    }

    #[test]
    fn integration_test_order_of_execution_parent_gives_birth() {
        let settings = Settings::zero();

        let mut world = World::new(3, 1, settings);
        world.set_cell_facing(1, 0, Genome::new_yeast(), Direction::West);
        world.set_cell_facing(2, 0, Genome::new_predator(), Direction::West);

        world.apply(&[
            Action::Reproduce { x: 1, y: 0 },
            Action::Attack {
                x: 2,
                y: 0,
                damage: 100,
            },
        ]);

        assert!(
            matches!(world.entity_at(1, 0), Entity::Corpse(_)),
            "parent should have been destroyed"
        );

        assert!(
            matches!(world.entity_at(0, 0), Entity::Cell(_)),
            "new life should have survived"
        );
    }
}
