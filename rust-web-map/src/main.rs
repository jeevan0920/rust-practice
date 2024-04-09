#[macro_use]
extern crate diesel;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use env_logger;
mod auth;
mod db;
mod models;
mod osm;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the logger
    env_logger::init();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new().wrap(cors).configure(routes::config)
        // Include other middleware and configurations
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
