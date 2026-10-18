use bevy::{
    prelude::*,
    ui_widgets::{observe, Activate},
};

use crate::{
    editor::events::EditorSave,
    GameState,
    ui::components,
};

pub struct UIEditorPlugin;

impl Plugin for UIEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Editor), (
            spawn_editor_ui,
        ));
    }
}

fn spawn_editor_ui(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![(
            components::spawn_button(&asset_server, "save", IVec2::new(150, 60)),
            components::ButtonAction::EditorSave,
        )],
    ));
}