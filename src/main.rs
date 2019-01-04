#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;

extern crate actix_web;
extern crate core;

use actix_web::{server, App, HttpRequest, Responder};
use std::env;

pub mod morphoid;
use morphoid::types::*;

use actix_web::fs;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

use core::mem;

lazy_static! {
    static ref PROCESSOR: Mutex<Processor> = Mutex::new(Processor::new());
    static ref WORLD: Mutex<World> = Mutex::new(build_new_world());
}

//  12345678
//1  00 00
//2 0000000
//3  00000
//4   000
//5    0

fn build_new_world() -> World {
    let mut world = World::new(20, 20, Settings::prod());
    let coords_vec = vec![
        (2,1), (3,1), (5,1),(6,1),
        (1,2), (2,2), (3,2), (4,2), (5,2), (6,2), (7,2),
        (2,3), (3,3), (4,3), (5,3), (6,3),
        (3,4), (4,4), (5,4),
        (4,5)
    ];
    for (x, y) in coords_vec.iter() {
        let mut genome = Genome::new_plant();
        genome.mutate(2, REPRODUCE);
        world.set_cell(*x + 7, *y + 7, genome);
    }
    world
}

fn world_state(_req: &HttpRequest) -> impl Responder {
    let result = format!("{}", WORLD.lock().unwrap());
    println!("Got result");
    result
}

fn reset_world(_req: &HttpRequest) -> impl Responder {
    let mut world = WORLD.lock().expect("Could not lock mutex");
    mem::replace(&mut *world, build_new_world());
    "OK"
}

fn initialize() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(500));
            WORLD.lock().unwrap().tick(&mut PROCESSOR.lock().unwrap());
        }
    });
}

fn main() {
    println!("Starting...");
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    initialize();

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .resource("/world", |r| r.f(world_state))
            .resource("/reset", |r| r.f(reset_world))
            .handler(
                "/",
                fs::StaticFiles::new("static/")
                    .unwrap()
                    .index_file("index.html")
            )
    })
    .bind(("0.0.0.0", port))
    .expect(&format!("Can not bind to port {:?}", port))
    .run();
}
