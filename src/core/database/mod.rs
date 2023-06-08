//! Database module
//!
//! This module implement Dragon Bot Z Portal Service repositories and models
//!
//! Authors: Lahcène Belhadi <lahcene.belhadi@gmail.com>
use dbzlib_rs::model::character::Character;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Portal {
    id: Option<i64>,
    name: String,
    description: String,
    image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct PortalContent {
    portal: Portal,
    characters: Vec<i64>,
}
