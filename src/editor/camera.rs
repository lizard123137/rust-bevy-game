use bevy::{
    input::mouse::AccumulatedMouseScroll,
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    editor::events::{
        EditorPlace,
        EditorRemove,
    },
    GameState,
};

const EDITOR_PLACE_MARGIN: f32 = 100.0;

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Editor), (
            spawn_camera,
        ));
        app.add_systems(Update, (
            camera_zoom
                .run_if(in_state(GameState::Editor)),
            camera_input_mouse
                .run_if(in_state(GameState::Editor))
        ));
    }
}

#[derive(Debug, Component)]
struct EditorCamera;

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        EditorCamera,
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

fn camera_input_mouse(
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<EditorCamera>>,
    mut commands: Commands,
    mut gizmos: Gizmos,
) {
    // Get mouse world position
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let resolution = &window.resolution;
    let window_size_x = resolution.width();
    let window_size_y = resolution.height();

    if cursor_position.x < EDITOR_PLACE_MARGIN ||
        cursor_position.x > window_size_x - EDITOR_PLACE_MARGIN ||
        cursor_position.y < EDITOR_PLACE_MARGIN ||
        cursor_position.y > window_size_y - EDITOR_PLACE_MARGIN {
        return;
    }

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    // Snap mouse position to grid
    let quantified_position = (world_position / 16.0).floor() * 16.0;

    // Debug placement square
    gizmos.rect_2d(
        quantified_position,
        Vec2::splat(16.0),
        Color::srgb(0.7, 0.2, 0.2),
    );

    if buttons.just_pressed(MouseButton::Left) {
        commands.trigger(EditorPlace { pos: quantified_position.extend(1.0), name: "grass".to_string() });
    }

    if buttons.just_pressed(MouseButton::Right) {
        commands.trigger(EditorRemove { pos: quantified_position.extend(1.0) });
    }
}