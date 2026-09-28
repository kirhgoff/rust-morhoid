use std::env;

use actix_web::{middleware, web, App, HttpServer};

use api::methods::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("actix_web=info"))
        .init();

    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting Morphoid on PORT={}", port);

    initialize_world();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/world/settings/get").route(web::get().to(api_get_settings)))
            .service(
                web::resource("/world/settings/update").route(web::post().to(api_update_settings)),
            )
            .service(web::resource("/world/get").route(web::get().to(api_get_world)))
            .service(web::resource("/entity/{x}/{y}").route(web::get().to(api_get_cell)))
            .service(web::resource("/world/reset").route(web::post().to(api_reset_world)))
            .service(actix_files::Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
