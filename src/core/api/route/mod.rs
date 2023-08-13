//! Portal service's API routes
//!
//! This module contais the implementation of Portal Service's API routes
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>

use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello and welcome to Dragon Bot Z's Portal Service!")
}
