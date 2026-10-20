use bevy::prelude::*;

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

fn spawn_editor_ui(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: px(12),
                    left: px(12),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![(
                    components::spawn_button("save", IVec2::new(150, 60)),
                    components::ButtonAction::EditorSave,
                )],
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    bottom: px(12),
                    right: px(12),
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                children![
                    (
                        components::spawn_button("Layer 2", IVec2::new(200, 60)),
                        components::ButtonAction::EditorLayerSelect(2.0),
                    ),
                    (
                        components::spawn_button("Layer 1", IVec2::new(200, 60)),
                        components::ButtonAction::EditorLayerSelect(1.0),
                    ),
                    (
                        components::spawn_button("Layer 0", IVec2::new(200, 60)),
                        components::ButtonAction::EditorLayerSelect(0.0),
                    ),
                    (
                        components::spawn_button("Layer -1", IVec2::new(200, 60)),
                        components::ButtonAction::EditorLayerSelect(-1.0),
                    ),
                    (
                        components::spawn_button("Layer -2", IVec2::new(200, 60)),
                        components::ButtonAction::EditorLayerSelect(-2.0),
                    )
                ],
            )
        ]
    ));
}