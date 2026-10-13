use bevy::{
    input::common_conditions::*,
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    audio::PlaySound,
    GameState,
    gameplay::{
        physics::Rigidbody,
        player::{
            animation::{
                TongueDestroy,
                TongueCreate,
            },
            frog_tongue_active,
            Frog,
            PlayerCamera,
        },
        raycasting::ray_get_collision_points_in_range,
        spatial_hash_grid::SpatialHashGrid,
    },
};

pub struct PlayerGrapplePlugin;

impl Plugin for PlayerGrapplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_target
                .run_if(in_state(GameState::Game))
                .run_if(not(frog_tongue_active)),
            activate_tongue
                .run_if(in_state(GameState::Game))
                .run_if(input_just_pressed(MouseButton::Right)),
            deactivate_tongue
                .run_if(in_state(GameState::Game))
                .run_if(input_just_released(MouseButton::Right)),
        ));
    }
}

fn update_target(
    mut frog: Single<&mut Frog>,
    window: Single<&Window, With<PrimaryWindow>>,
    cameras: Query<(&Camera, &GlobalTransform), With<PlayerCamera>>,
) {
    let Ok((camera, camera_transform)) = cameras.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let Ok(world_position) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        return;
    };

    frog.target_pos = world_position;
}



#[derive(Debug, Component)]
pub struct Tongue;

fn activate_tongue(
    mut commands: Commands,
    mut frog: Single<&mut Frog>,
    query: Query<(Entity, &Transform, &Rigidbody)>,
    transform: Single<&Transform, With<Frog>>,
    grid: Res<SpatialHashGrid>,
) {
    frog.tongue_active = true;
    frog.tongue_len = transform.translation
        .truncate()
        .distance(frog.target_pos);

    commands.trigger(TongueCreate { origin: transform.translation.truncate(), target: frog.target_pos});
    commands.trigger(PlaySound { name: "grapple".to_string() });

    // Draw lines in range
    ray_get_collision_points_in_range(
        &grid,
        transform.translation.truncate(),
        (frog.target_pos - transform.translation.truncate()).normalize_or_zero(),
        64.0,
        &query
    );
}

fn deactivate_tongue(
    mut commands: Commands,
    mut frog: Single<&mut Frog>,
) {
    frog.tongue_active = false;
    commands.trigger(TongueDestroy);
}