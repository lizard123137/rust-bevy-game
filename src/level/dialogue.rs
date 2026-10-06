use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, Clone)]
pub struct DialogueRegistry {
    pub dialogues: HashMap<String, DialogueDef>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DialogueDef {
    pub sprite: String,
}

impl DialogueRegistry {
    pub fn get(&self, name: &str) -> Option<&DialogueDef> {
        self.dialogues.get(name)
    }
}