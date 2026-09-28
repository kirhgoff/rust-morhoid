use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use actix_web::web::{Json, Path};
use actix_web::HttpResponse;

use serde::Deserialize;

use crate::types::*;
use crate::{WORLD_HEIGHT, WORLD_WIDTH};
use morphoid::{Coords, Settings, World};

const SLEEP_BETWEEN_TICKS: u64 = 25;

static WORLD: LazyLock<Mutex<World>> = LazyLock::new(|| Mutex::new(new_world()));

fn world() -> MutexGuard<'static, World> {
    WORLD.lock().unwrap_or_else(PoisonError::into_inner)
}

fn new_world() -> World {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u64)
        .unwrap_or_default();
    World::random(WORLD_WIDTH, WORLD_HEIGHT, Settings::prod(), seed)
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
    if let Err(message) = json.validate() {
        return HttpResponse::BadRequest().body(message);
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
    Json(CellInfo::at(&world(), coords.x, coords.y))
}
