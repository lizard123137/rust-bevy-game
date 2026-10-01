use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, Clone)]
pub struct TileRegistry {
    pub tiles: HashMap<String, TileDef>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TileDef {
    pub sprite: String,
}