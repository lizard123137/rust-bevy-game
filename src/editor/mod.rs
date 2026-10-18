mod camera;
mod change_list;
pub mod events;
pub mod file_handler;

use camera::EditorCameraPlugin;
use events::EditorEventsPlugin;

pub use file_handler::file_changes_apply;

use bevy::prelude::*;

use crate::{
    GameState,
    editor::change_list::EditorChangeList,
    level::level_loader,
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
        app.add_plugins((
            EditorCameraPlugin,
            EditorEventsPlugin,
        ));
        app.add_systems(OnEnter(GameState::Editor), (
            level_loader::spawn_level,
        ));
    }
}

