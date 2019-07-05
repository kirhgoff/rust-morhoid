use actix_web::*;

use std::env;

use api::methods::*;


fn main() -> std::io::Result<()> {
    println!("------------------------------------");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let port:u32 = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    println!("Starting Morphoid on PORT={:?}", port);

    initialize_world();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/reset").route(web::get().to(api_reset_world)))
            .service(web::resource("/world/get").route(web::get().to(api_get_world)))
            .service(web::resource("/entity/{x}/{y}").route(web::get().to(api_get_cell)))
            .service(
                actix_files::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind(format!("0.0.0.0:{:?}", port))?
    .run()
}
