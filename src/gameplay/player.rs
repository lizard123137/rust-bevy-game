use bevy::{
    camera::visibility::RenderLayers,
    color::palettes::css::*,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    input::common_conditions::*,
    image::{ImageLoaderSettings, ImageSampler},
    post_process::bloom::Bloom,
    prelude::*,
    window::PrimaryWindow,
};

use crate::effects::lofi::LofiEffect;
use crate::gameplay::physics::Rigidbody;

pub struct PlayerPlugin;

pub fn frog_tongue_active(query: Query<&Frog>) -> bool {
    query.single().map_or(false, |frog| frog.tongue_active)
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (
            update_target
                .run_if(not(frog_tongue_active)),
            update_tongue
                .run_if(frog_tongue_active),

            update_orientation,

            activate_jump.run_if(input_just_pressed(MouseButton::Left)),

            activate_tongue.run_if(input_just_pressed(MouseButton::Right)),
            deactivate_tongue.run_if(input_just_released(MouseButton::Right)),
        ));
        app.add_systems(FixedUpdate, (
            debug_player_gizmos,
        ));
    }
}

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Debug, Component)]
pub struct Frog {
    pub grounded: bool,
    pub tongue_active: bool,

    pub target_pos: Vec2,
    pub tongue_len: f32,
    pub range: f32,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let frog_image = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        })
        .load("images/frog.png");

    commands.spawn((
        Rigidbody::default(),
        Frog {
            grounded: false,
            tongue_active: false,
            target_pos: Vec2::new(0.0, 0.0),
            tongue_len: 0.0,
            range: 50.0,
        },
        Sprite::from_image(frog_image),
        Transform::from_xyz(0.0, 50.0, 0.0),
        children![
            (
                Camera2d,
                Camera {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..default()
                },
                LofiEffect::new(2.0, 15.0),
                Tonemapping::TonyMcMapface,
                Bloom::default(),
                DebandDither::Enabled,
                PlayerCamera,
                RenderLayers::from_layers(&[0, 1]), // scene + water
                Transform::from_scale(Vec3::splat(0.25)), // TODO zoom with scroll
            )
        ],
    ));
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

fn update_orientation(query: Single<(&mut Sprite, &Rigidbody), With<Frog>>) {
    let (mut sprite, rb) = query.into_inner();

    if rb.velocity.x >= 0.0 {
        sprite.flip_x = false;
    } else {
        sprite.flip_x = true;
    }
}

fn activate_jump(query: Single<(&mut Rigidbody, &Transform, &mut Frog)>) {
    let (mut rb, t, mut frog) = query.into_inner();

    if !frog.grounded {
        return;
    }

    let mut direction = frog.target_pos - t.translation.truncate();
    let magnitude = direction.length();
    direction /= magnitude; // Normalize

    let jump_strength = magnitude.clamp(0.0, frog.range);

    rb.force += direction * jump_strength * 200.0;
    
    frog.grounded = false;
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
    let direction = target_transform.translation - transform.translation;
    let angle = direction.y.atan2(direction.x);

    let tongue = meshes.add(Rectangle::new(distance, 2.0));
    let color = materials.add(Color::srgb(0.2, 0.5, 0.0));     

    frog.tongue_len = distance;

    sprite.image = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        })
        .load("images/frog_open.png");

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
}

fn deactivate_tongue(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut frog: Single<&mut Frog>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
    query: Query<Entity, With<Tongue>>
) {
    frog.tongue_active = false;

    sprite.image = asset_server
        .load_builder()
        .with_settings(|settings: &mut ImageLoaderSettings| {
            settings.sampler = ImageSampler::nearest();
        })
        .load("images/frog.png");

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn debug_player_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Frog)>
) {
    for (transform, frog) in query {
        // Draw range
        gizmos
            .circle_2d(transform.translation.truncate(), frog.range, PURPLE)
            .resolution(64);

        // Draw target
        gizmos.line_2d(transform.translation.truncate(), frog.target_pos, PURPLE);
    }
}