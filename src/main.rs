// #[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate json;

extern crate actix_web;
extern crate core;
extern crate rand;
extern crate futures;

use std::env;

pub mod morphoid;
use morphoid::types::*;

use actix_web::*;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;

use core::mem;

use rand::{Rng};
use rand::prelude::ThreadRng;
use actix_web::http::ContentEncoding;
use actix_web::middleware::Logger;
use actix_web::{
    http::header, middleware::cors::Cors, App, HttpServer,
};

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
                let mut genome = create_random_entity(&mut rng);
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

fn world_state(_req: &HttpRequest) -> impl Responder {
    format!("{}", WORLD.lock().unwrap())
}

fn reset_world(_req: &HttpRequest) -> impl Responder {
    let mut world = WORLD.lock().expect("Could not lock mutex");
    mem::replace(&mut *world, build_new_world());
    "OK"
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WorldView {
    width: i32,
    height: i32,
    data: String,
}

pub fn api_get_world() -> web::Json<WorldView> {
    let world = WORLD.lock().unwrap();

    web::Json(WorldView {
        width: world.width,
        height: world.height,
        data: format!("{}", world),
    })
}

//fn api_get_world(_req: &HttpRequest) -> HttpResponse {
//    let world = WORLD.lock().unwrap();
//
//    let out_json = json::object! {
//        "width" => world.width,
//        "height" => world.height,
//        "data" => format!("{}", world),
//    };
//
//    let response = out_json.dump();
//
//    //println!("Sending: {:?}", response);
//
//    HttpResponse::Ok()
//        .content_type("application/json")
//        .body(response)
//}

fn initialize() {
    thread::spawn(|| {
        loop {
            // TODO: wtf?! is it really a solution?
            thread::sleep(Duration::from_millis(700));
            WORLD.lock().unwrap().tick(&mut PROCESSOR.lock().unwrap());
        }
    });
}

fn main() {
    println!("Starting morphoid.");
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    initialize();

    // Start a server, configuring the resources to serve.
    HttpServer::new(move || {
        App::new()
            .wrap(
            Cors::new()
                .allowed_origin("*")
                .allowed_methods(vec!["GET", "POST"])
                .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                .allowed_header(header::CONTENT_TYPE)
                .max_age(3600)
            )
            .wrap(Logger::default())
            .service(web::resource("/world/get").route(web::get().to(api_get_world)))
    })
    .bind(("0.0.0.0", port))
    .expect(&format!("Can not bind to port {:?}", port))
    .run();
}
