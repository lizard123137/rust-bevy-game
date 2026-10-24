mod camera;
mod change_list;
pub mod events;
pub mod file_handler;
mod state;

use camera::EditorCameraPlugin;
use events::EditorEventsPlugin;

use bevy::prelude::*;

use crate::{
    GameState,
    editor::{
        change_list::EditorChangeList,
        state::EditorState,
    },
    level::{
        level_loader,      
        level_loader::ObjectType,
    },
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        // TODO impl default trait and open file with file browser on startup
        app.insert_resource(EditorChangeList {
            file_name: "assets/levels/level_temp.level.ron".to_string(),
            level_name: "test".to_string(),
            level_description: "test".to_string(),
            changes: Vec::new(),
        });
        app.insert_resource(EditorState {
            selected_layer: 0.0,
            selected_object: ObjectType::Tile {
                name: "grass".to_string(),
                solid: true,
            }
        });
        app.add_plugins((
            EditorCameraPlugin,
            EditorEventsPlugin,
        ));
        app.add_systems(OnEnter(GameState::Editor), (
            level_loader::spawn_level,
        ));
    }
}

