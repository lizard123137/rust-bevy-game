use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
};
use crate::{
    asset_loader::GameAssets,
    level::level_loader,
    GameState
};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Editor), (
            spawn_camera,
        ));
        app.add_systems(Update, (
            level_loader::spawn_level
                .run_if(in_state(GameState::Editor))
                .run_if(resource_exists::<GameAssets>),
            camera_zoom
                .run_if(in_state(GameState::Editor)),
        ));
    }
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
    ));
}

fn camera_zoom(
    accumulated_scroll: Res<AccumulatedMouseScroll>,
    mut query: Query<&mut Projection, With<Camera2d>>,
) {
    if accumulated_scroll.delta.y == 0.0 {
        return;
    }

    let zoom_sensitivity = 0.1;
    let min_zoom = 0.2; // Max zoom in
    let max_zoom = 5.0; // Max zoom out

    for mut projection in &mut query {
        if let Projection::Orthographic(ref mut ortho) = *projection {
            let zoom_factor = 1.0 - accumulated_scroll.delta.y * zoom_sensitivity;
            ortho.scale = (ortho.scale * zoom_factor).clamp(min_zoom, max_zoom);
        }
    }
}