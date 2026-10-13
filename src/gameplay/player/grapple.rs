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
            update_tongue
                .run_if(in_state(GameState::Game))
                .run_if(frog_tongue_active),
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

fn update_tongue(
    mut tongue_transform: Single<&mut Transform, With<Tongue>>,
    query: Single<(&Frog, &Transform), Without<Tongue>>,
) {
    let (frog, transform) = *query;

    let target_transform = Transform::from_xyz(
            frog.target_pos.x,
            frog.target_pos.y,
            0.0);

    let middle = (transform.translation + target_transform.translation) / 2.0;
    let direction = target_transform.translation - transform.translation;
    let angle = direction.y.atan2(direction.x);

    tongue_transform.translation = middle;
    tongue_transform.rotation = Quat::from_rotation_z(angle);
}


#[derive(Debug, Component)]
struct Tongue;

fn activate_tongue(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    mut frog: Single<&mut Frog>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
    transform: Single<&Transform, With<Frog>>,
    query: Query<(Entity, &Transform, &Rigidbody)>,
    grid: Res<SpatialHashGrid>,
) {
    frog.tongue_active = true;

    let target_transform = Transform::from_xyz(
            frog.target_pos.x,
            frog.target_pos.y,
            0.0);
    let distance = transform
        .translation
        .distance(target_transform.translation);

    let middle = (transform.translation + target_transform.translation) / 2.0;
    let direction = (target_transform.translation - transform.translation)
        .truncate()
        .normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    let tongue = meshes.add(Rectangle::new(distance, 2.0));
    let color = materials.add(Color::srgb(0.2, 0.5, 0.0));     

    frog.tongue_len = distance;

    sprite.image = asset_server.load("images/frog_open.png");

    commands.spawn(
        (
            Tongue,
            Mesh2d(tongue),
            MeshMaterial2d(color),
            Transform {
                translation: middle,
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
        )
    );

    commands.trigger(PlaySound { name: "grapple".to_string() });

    // Draw lines in range
    ray_get_collision_points_in_range(
        &grid,
        transform.translation.truncate(),
        direction,
        64.0,
        &query
    );
}

fn deactivate_tongue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut frog: Single<&mut Frog>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
    query: Query<Entity, With<Tongue>>
) {
    frog.tongue_active = false;

    sprite.image = asset_server.load("images/frog.png");

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}