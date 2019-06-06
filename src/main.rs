// #[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
extern crate json;

extern crate actix_web;
extern crate actix_files;
extern crate core;
extern crate rand;
extern crate futures;

pub mod morphoid;
use crate::morphoid::types::*;

use actix_web::*;
use std::{thread, env};
use std::time::Duration;
use std::sync::Mutex;

use core::mem;

use rand::{Rng};
use rand::prelude::ThreadRng;

use actix_files as fs;

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

fn api_reset_world(_req: HttpRequest) -> impl Responder {
    let mut world = WORLD.lock().expect("Could not lock mutex");
    mem::replace(&mut *world, build_new_world());
    HttpResponse::Ok()
}

fn api_get_world(_req: HttpRequest) -> HttpResponse {
    let world = WORLD.lock().unwrap();

    let out_json = json::object! {
        "width" => world.width,
        "height" => world.height,
        "data" => format!("{}", world),
    };

    let response = out_json.dump();

    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}

fn initialize_world() {
    thread::spawn(|| {
        loop {
            // TODO: wtf?! is it really a solution?
            thread::sleep(Duration::from_millis(500));
            WORLD.lock().unwrap().tick(&mut PROCESSOR.lock().unwrap());
        }
    });
}

fn main() -> std::io::Result<()> {
    let port_var = env::var("PORT");
    println!("PORT var is {:?}", port_var);

    let port:u32 = port_var
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting morphoid on PORT={:?}", port);

    initialize_world();

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/reset").route(web::get().to(api_reset_world)))
            .service(web::resource("/world/get").route(web::get().to(api_get_world)))
            .service(
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind(format!("0.0.0.0:{:?}", port))?
    .run()
}
