use bevy::prelude::*;

use crate::level::level_loader::ObjectType;

#[derive(Resource)]
pub struct EditorState {
    pub selected_layer: f32,
    pub selected_object: ObjectType,
}