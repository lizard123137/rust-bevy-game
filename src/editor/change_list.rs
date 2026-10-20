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
    pub object: ObjectData,
}

// Maybe convert to use enums that store the objects or what to remove
pub enum EditorChangeType {
    Add,
    Remove,
}