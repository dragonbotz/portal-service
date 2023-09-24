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

    // retrieve database user
    let mut pg_user = String::new();
    if let Ok(pg_user_path) = std::env::var("PG_USER") {
        if let Ok(user) = std::fs::read_to_string(pg_user_path) {
            pg_user = user;
        }
    }

    let mut pg_password = String::new();
    if let Ok(pg_password_path) = std::env::var("DBZ_PG_SUPERUSER_PASS") {
        if let Ok(password) = std::fs::read_to_string(pg_password_path) {
            pg_password = password;
        }
    }

    // connect the databse
    let database = PgDatabase::new(
        format!("postgresql://postgres:{pg_password}@database:5433/portaldb").as_str(),
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
    .bind(("0.0.0.0", 8081));

    if let Err(error) = server {
        panic!("An error occured while binding server to ip adress and port: {error}")
    }

    // Runs the server
    let running_server = server.unwrap().run().await;
    if let Err(error) = running_server {
        panic!("An error occured while running the server: {error}")
    }
}
