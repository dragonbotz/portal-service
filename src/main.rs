//! Dragon Bot Z Portal Service's entry point
//!
//! This module contains the implementation of the entry point of
//! Dragon Bot Z Portal Service
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;

use actix_web::{web, App, HttpServer};
use crate::core::database::PortalRepository;
use dbzlib_rs::database::PgDatabase;
use dbzlib_rs::util::error::Error;

#[actix_web::main]
async fn main() {
    println!("Running server on: http://127.0.0.1:8081/");

    // connect the databse
    let database = PgDatabase::new("postgresql://postgres@database:5433/portaldb").await;

    if let Err(error) = database {
        panic!("{}", Error::DatabaseConnection(error.to_string()))
    }

    let database = database.unwrap();
    let repository = PortalRepository::new(&database);
    let portal_result = repository.get(1).await;

    if portal_result.is_err() {
        panic!("No portal found, or there might be an error");
    } else if portal_result.is_ok() {
        println!("A portal has been found");
    }

    // Setup server
    let server = HttpServer::new(move || App::new().app_data(web::Data::new(database.clone())))
        .bind(("127.0.0.1", 8081));

    if let Err(error) = server {
        panic!("An error occured while binding server to ip adress and port: {error}")
    }

    // Runs the server
    let running_server = server.unwrap().run().await;
    if let Err(error) = running_server {
        panic!("An error occured while running the server: {error}")
    }
}
