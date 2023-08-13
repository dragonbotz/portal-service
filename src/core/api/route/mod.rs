//! Portal service's API routes
//!
//! This module contais the implementation of Portal Service's API routes
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use actix_web::{get, web, HttpResponse, Responder};
use dbzlib_rs::database::PgDatabase;

use crate::core::database::PortalRepository;

/// Returns an Hellow world message
#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's Portal Service!")
}

/// Returns a portal data if found, 404 Not Found otherwise
#[get("/get-data/{id}")]
async fn get_data(database: web::Data<PgDatabase>, id: web::Path<i64>) -> impl Responder {
    let repository = PortalRepository::new(&database);
    let id = id.into_inner();

    let portal_data = repository.get_data(id).await;
    if portal_data.is_err() {
        return HttpResponse::NotFound().body(format!("Portal #{} not found", id));
    }

    let portal_data = portal_data.unwrap();
    HttpResponse::Ok().json(portal_data)
}

/// Returns a portal content if found, 404 Not Found otherwise
#[get("/get-content/{id}")]
async fn get_content(database: web::Data<PgDatabase>, id: web::Path<i64>) -> impl Responder {
    let repository = PortalRepository::new(&database);
    let id = id.into_inner();

    let characters = repository.get_content(id).await;
    if let Err(error) = characters {
        return HttpResponse::NotFound().body(error.to_string());
    }
    let characters = characters.unwrap();

    HttpResponse::Ok().json(characters)
}
