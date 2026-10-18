use bevy::prelude::*;

use crate::level::level_loader::{
    ObjectData,
};

#[derive(Resource)]
pub struct EditorChangeList {
    pub file_name: String,
    pub level_name: String,
    pub level_description: String,
    pub changes: Vec<EditorChange>
}

pub struct EditorChange {
    pub change_type: EditorChangeType,
    pub object: Option<ObjectData>,
}

pub enum EditorChangeType {
    Add,
    Remove,
}