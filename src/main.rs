#[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;

extern crate actix_web;

use actix_web::{server, App, HttpRequest, Responder};
use std::env;

pub mod morphoid;
use morphoid::world::World;
use morphoid::processor::Processor;

use actix_web::fs;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(World::new(10, 10));
    static ref PROCESSOR: Processor = Processor::new();
}

fn world_state(_req: &HttpRequest) -> impl Responder {
    format!("{}", WORLD.lock().unwrap())
}

fn initialize() {
    thread::spawn(|| {
        loop {
            thread::sleep(Duration::from_millis(30));
            WORLD.lock().unwrap().tick(&PROCESSOR);
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
            .handler("/", fs::StaticFiles::new("static/").unwrap())
    })
    .bind(("0.0.0.0", port))
    .expect(&format!("Can not bind to port {:?}", port))
    .run();
}
