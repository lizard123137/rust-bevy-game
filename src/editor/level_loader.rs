use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, Clone)]
pub struct LevelData {
    pub name: String,
    pub description: String,
    pub objects: Vec<ObjectData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectData {
    pub obj_type: ObjectType,
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectType {
    Tile {name: String, solid: bool},
    Mushroom,
    Player,
}