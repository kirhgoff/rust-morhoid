use lazy_static::lazy_static;

use std::thread;
use std::time::Duration;
use std::sync::Mutex;

use core::mem;

use rand::{Rng};
use rand::prelude::ThreadRng;

use actix_web::*;
use actix_web::web::{Json, Path};

use serde::{Deserialize};

use morphoid::types::*;
use crate::types::*;

lazy_static! {
    static ref PROCESSOR: Mutex<Processor> = Mutex::new(Processor::new());
    static ref WORLD: Mutex<World> = Mutex::new(build_new_world());
}

fn build_new_world() -> World {
    let mut rng = rand::thread_rng();

    let settings = Settings ::prod();
    let width = 40;
    let height = 40;
    let mut world = World::new(width, height, settings);

    for x in 0..width {
        for y in 0..height {
            if rng.gen_ratio(1,3) {
                let genome = create_random_entity(&mut rng);
                let direction = Direction::by_value(rng.gen_range(0, 8));

                world.set_cell_ext(x, y, genome, direction);
            } else {
                world.set_nothing(x, y);
            }
        }
    }
    world
}

fn create_random_entity(rng: &mut ThreadRng) -> Genome {
    let mut genome = Genome::new_plant();
    let mut i = 0;
    while i < GENOME_LENGTH - 4 {
        let index = rng.gen_range(0, KNOWN_GENES.len());
        let gene = KNOWN_GENES[index];
        genome.mutate(i, gene);
        i += 1;

        if gene == SENSE {
            for _ in 0..2 {
                genome.mutate(i, rng.gen_range(0, GENOME_LENGTH));
                i += 1;
            }
        }
        if gene == TURN {
            genome.mutate(i, rng.gen_range(0, GENOME_LENGTH));
            i += 1;
        }
    }
    genome
}

pub fn initialize_world() {
    thread::spawn(|| {
        loop {
            // TODO: wtf?! is it really a solution?
            thread::sleep(Duration::from_millis(50));
            WORLD.lock().unwrap()
                .tick(&mut PROCESSOR.lock().unwrap());
        }
    });
}

pub fn api_reset_world(_req: HttpRequest) -> impl Responder {
    let mut world = WORLD.lock().expect("Could not lock mutex");

    mem::replace(&mut *world, build_new_world());
    HttpResponse::Ok()
}

pub fn api_get_world(_req: HttpRequest) -> Result<Json<WorldInfo>> {
    let world = WORLD.lock().unwrap();
    let projection = GeneTypesProjection {};

    Ok(Json(WorldInfo::from(&world, &projection)))
}

#[derive(Debug, Deserialize)]
pub struct CellCoordsParams { x: Coords, y: Coords }

pub fn api_get_cell(path: Path<CellCoordsParams>) -> Result<Json<Option<CellInfo>>> {
    let coords = path.into_inner();
    let world = WORLD.lock().unwrap();

    let info = match world.get_entity(coords.x, coords.y) {
        Entity::Cell(genome_id) => {
            let entity_state = world.get_state(*genome_id);
            let genome = world.get_genome(*genome_id).unwrap();

            Some(CellInfo {
                x: coords.x,
                y: coords.y,
                health: entity_state.health,
                direction: entity_state.direction as usize,
                genome_id: genome.id,
                genome: genome.genes.to_vec()

            })
        },
        _ => None
    };

    Ok(Json(info))
}