//! Dragon Bot Z Portal Service's entry point
//!
//! This module contains the implementation of the entry point of
//! Dragon Bot Z Portal Service
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;

use crate::core::api::route;
use actix_web::{web, App, HttpServer};
use dbzlib_rs::database::PgDatabase;
use dbzlib_rs::util::error::Error;

#[actix_web::main]
async fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let mut pg_password = String::new();
    if let Ok(pass) = std::env::var("DBZ_PG_SUPERUSER_PASS") {
        pg_password = pass;
    }

    // connect the databse
    let database = PgDatabase::new(
        format!("postgresql://postgres:{pg_password}@dbz-portal-database:5433/portaldb").as_str(),
    )
    .await;

    if let Err(error) = database {
        panic!("{}", Error::DatabaseConnection(error.to_string()))
    }

    let database = database.unwrap();

    // Setup server
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(database.clone()))
            .service(route::root)
            .service(route::get_data)
            .service(route::get_content)
    })
    .bind(("0.0.0.0", 8080));

    if let Err(error) = server {
        panic!("An error occured while binding server to ip adress and port: {error}")
    }

    // Runs the server
    let running_server = server.unwrap().run().await;
    if let Err(error) = running_server {
        panic!("An error occured while running the server: {error}")
    }
}
