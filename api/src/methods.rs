use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::Duration;

use actix_web::web::{Json, Path};
use actix_web::HttpResponse;

use serde::Deserialize;

use crate::types::*;
use morphoid::{Coords, Entity, Settings, World};

const SLEEP_BETWEEN_TICKS: u64 = 25;
const WORLD_WIDTH: Coords = 40;
const WORLD_HEIGHT: Coords = 40;

static WORLD: LazyLock<Mutex<World>> = LazyLock::new(|| Mutex::new(new_world()));

fn world() -> MutexGuard<'static, World> {
    WORLD.lock().unwrap_or_else(PoisonError::into_inner)
}

fn new_world() -> World {
    World::random(WORLD_WIDTH, WORLD_HEIGHT, Settings::prod())
}

pub fn initialize_world() {
    thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(SLEEP_BETWEEN_TICKS));
        world().tick();
    });
}

pub async fn api_reset_world() -> HttpResponse {
    *world() = new_world();
    HttpResponse::Ok().finish()
}

pub async fn api_get_settings() -> Json<SettingsInfo> {
    Json(SettingsInfo::from(&world().settings))
}

pub async fn api_update_settings(json: Json<SettingsInfo>) -> HttpResponse {
    if !(0.0..=1.0).contains(&json.mutation_probability) {
        return HttpResponse::BadRequest().body("mutation_probability must be within [0, 1]");
    }
    world().settings = Settings::from(&*json);
    HttpResponse::Ok().finish()
}

pub async fn api_get_world() -> Json<WorldInfo> {
    Json(WorldInfo::from(&*world()))
}

#[derive(Debug, Deserialize)]
pub struct CellCoordsParams {
    x: Coords,
    y: Coords,
}

pub async fn api_get_cell(path: Path<CellCoordsParams>) -> Json<Option<CellInfo>> {
    let coords = path.into_inner();
    let world = world();

    let info = match world.entity_at(coords.x, coords.y) {
        Entity::Cell(genome_id) => {
            let cell = world.cell(genome_id);

            Some(CellInfo {
                x: coords.x,
                y: coords.y,
                health: cell.health,
                direction: cell.direction as usize,
                genome_id: cell.genome.id,
                genome: cell.genome.genes.to_vec(),
            })
        }
        _ => None,
    };

    Json(info)
}
