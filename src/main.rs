//! Dragon Bot Z Portal Service's entry point
//!
//! This module contains the implementation of the entry point of 
//! Dragon Bot Z Portal Service
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

mod core;

use actix_web::{web, HttpServer, App};

#[actix_web::main]
async fn main() {
    println!("Running server on: http://127.0.0.1:8081/"); 

    // Setup server
    let server = HttpServer::new(move || {
        App::new()
    })
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
