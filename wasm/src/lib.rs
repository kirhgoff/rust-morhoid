#![allow(clippy::missing_safety_doc)]

use api::types::{CellInfo, SettingsInfo, WorldInfo};
use api::{WORLD_HEIGHT, WORLD_WIDTH};
use morphoid::{Coords, Settings, World};

pub struct Sim {
    world: World,
    seed: u64,
    json: Vec<u8>,
}

impl Sim {
    fn store(&mut self, json: Vec<u8>) -> *const u8 {
        self.json = json;
        self.json.as_ptr()
    }
}

fn random_world(seed: u64) -> World {
    World::random(WORLD_WIDTH, WORLD_HEIGHT, Settings::prod(), seed)
}

#[unsafe(no_mangle)]
pub extern "C" fn mh_new(seed: u32) -> *mut Sim {
    let seed = seed as u64;
    Box::into_raw(Box::new(Sim {
        world: random_world(seed),
        seed,
        json: Vec::new(),
    }))
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_tick(sim: *mut Sim) {
    (*sim).world.tick();
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_reset(sim: *mut Sim) {
    let sim = &mut *sim;
    sim.seed = sim.seed.wrapping_add(1);
    sim.world = random_world(sim.seed);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_world_json(sim: *mut Sim) -> *const u8 {
    let sim = &mut *sim;
    let json = serde_json::to_vec(&WorldInfo::from(&sim.world)).unwrap();
    sim.store(json)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_settings_json(sim: *mut Sim) -> *const u8 {
    let sim = &mut *sim;
    let json = serde_json::to_vec(&SettingsInfo::from(&sim.world.settings)).unwrap();
    sim.store(json)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_cell_json(sim: *mut Sim, x: Coords, y: Coords) -> *const u8 {
    let sim = &mut *sim;
    let json = serde_json::to_vec(&CellInfo::at(&sim.world, x, y)).unwrap();
    sim.store(json)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_json_len(sim: *mut Sim) -> u32 {
    (*sim).json.len() as u32
}

#[unsafe(no_mangle)]
pub extern "C" fn mh_alloc(len: u32) -> *mut u8 {
    let mut buf = vec![0u8; len as usize];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mh_update_settings(sim: *mut Sim, ptr: *mut u8, len: u32) -> u32 {
    let bytes = Vec::from_raw_parts(ptr, len as usize, len as usize);
    let Ok(info) = serde_json::from_slice::<SettingsInfo>(&bytes) else {
        return 0;
    };
    if info.validate().is_err() {
        return 0;
    }
    (*sim).world.settings = Settings::from(&info);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    unsafe fn json(sim: *mut Sim, ptr: *const u8) -> String {
        let len = mh_json_len(sim) as usize;
        String::from_utf8(std::slice::from_raw_parts(ptr, len).to_vec()).unwrap()
    }

    #[test]
    fn exports_round_trip() {
        unsafe {
            let sim = mh_new(7);
            mh_tick(sim);

            let world: WorldInfo = serde_json::from_str(&json(sim, mh_world_json(sim))).unwrap();
            assert_eq!(world.data.len(), 1600);

            let settings = json(sim, mh_settings_json(sim));
            assert!(!json(sim, mh_cell_json(sim, 0, 0)).is_empty());

            let rejected = settings.replace(
                "\"mutation_probability\":0.5",
                "\"mutation_probability\":2.0",
            );
            let ptr = mh_alloc(rejected.len() as u32);
            std::ptr::copy_nonoverlapping(rejected.as_ptr(), ptr, rejected.len());
            assert_eq!(mh_update_settings(sim, ptr, rejected.len() as u32), 0);

            let accepted = settings.replace(
                "\"mutation_probability\":0.5",
                "\"mutation_probability\":0.1",
            );
            let ptr = mh_alloc(accepted.len() as u32);
            std::ptr::copy_nonoverlapping(accepted.as_ptr(), ptr, accepted.len());
            assert_eq!(mh_update_settings(sim, ptr, accepted.len() as u32), 1);
            assert!(json(sim, mh_settings_json(sim)).contains("\"mutation_probability\":0.1"));

            mh_reset(sim);
            drop(Box::from_raw(sim));
        }
    }
}
