use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, Clone)]
pub struct ItemRegistry {
    pub items: HashMap<String, ItemDef>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDef {
    pub sprite: String,
    // TODO add effect field
}

impl ItemRegistry {
    pub fn get(&self, name: &str) -> Option<&ItemDef> {
        self.items.get(name)
    }
}