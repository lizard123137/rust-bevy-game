pub mod components;
pub mod editor;
pub mod game;
pub mod menu;

use components::UIComponentsPlugin;
use editor::UIEditorPlugin;
use game::UIGamePlugin;
use menu::UIMenuPlugin;

use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UIComponentsPlugin,
            UIEditorPlugin,
            UIGamePlugin,
            UIMenuPlugin,
        ));
    }
}