// #[macro_use] extern crate itertools;
#[macro_use] extern crate lazy_static;
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

fn api_get_world(_req: &HttpRequest) -> HttpResponse {
    let world = WORLD.lock().unwrap();

    let out_json = json::object! {
        "width" => world.width,
        "height" => world.height,
        "data" => format!("{}", world),
    };

    let response = out_json.dump();

    //println!("Sending: {:?}", response);

    HttpResponse::Ok()
        // TODO: remove this - dangerous
        .header("Access-Control-Allow-Origin", "*")
        .content_type("application/json")
        .body(response)
}

fn initialize() {
    thread::spawn(|| {
        loop {
            // TODO: wtf?! is it really a solution?
            thread::sleep(Duration::from_millis(700));
            WORLD.lock().unwrap().tick(&mut PROCESSOR.lock().unwrap());
        }
    });
}

//fn test() {
//    let resp = test::TestRequest::with_header("content-type", "text/plain")
//        .run(&api_get_world)
//        .unwrap();
//    assert_eq!(resp.status(), http::StatusCode::OK);
//}

const INDEX_CSS: &str = include_str!("../ui/index.css");
const INDEX_HTML: &str = include_str!("../ui/index.html");
const BUNDLE_JS: &str = include_str!("../ui/bundle.js");

fn main() {
    println!("Starting morphoid.");
    
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    initialize();

    // Start a server, configuring the resources to serve.
    server::new(|| {
        App::new()
            .default_encoding(ContentEncoding::Identity)
            // API
            .resource("/world", |r| r.f(world_state))
            .resource("/reset", |r| r.f(reset_world))
            // TODO: properly name, add parameter https://docs.rs/actix-web/0.6.1/actix_web/struct.Path.html
            .resource("/world/get", |r| r.f(api_get_world))

            // Static part
            .resource("/bundle.js", |r| r.f(|_| {
                HttpResponse::Ok().content_type("text/javascript").body(BUNDLE_JS)
            }))
            .resource("/index.css", |r| r.f(|_| {
                HttpResponse::Ok().content_type("text/css").body(INDEX_CSS)
            }))
            .resource("/index.html", |r| r.f(|_| {
                HttpResponse::Ok().content_type("text/html").body(INDEX_HTML)
            }))
            .resource("/", |r| r.f(|_| {
                HttpResponse::Ok().content_type("text/html").body(INDEX_HTML)
            }))
    })
    .bind(("0.0.0.0", port))
    .expect(&format!("Can not bind to port {:?}", port))
    .run();
}
