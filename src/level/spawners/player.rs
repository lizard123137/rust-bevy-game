use bevy::{
    camera::visibility::RenderLayers,
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    post_process::bloom::Bloom,
    prelude::*,
};

use crate::{
    graphics::effects::lofi::LofiEffect,
    gameplay::{
        player::{Frog, PlayerCamera},
        physics::Rigidbody,
    }
};

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    pos: Vec3,
    spawn_camera: bool,
) {
    let frog_id = commands.spawn((
        Rigidbody::default(),
        Frog {
            grounded: false,
            tongue_active: false,
            target_pos: Vec2::ZERO,
            tongue_pos: Vec2::ZERO,
            tongue_len: 0.0,
            range: 50.0,
        },
        Sprite::from_image(asset_server.load("images/frog.png")),
        Transform::from_xyz(pos.x, pos.y, pos.z),
    )).id();
        
    if spawn_camera {
        let camera_id = commands.spawn((
            Camera2d,
            Camera {
                clear_color: ClearColorConfig::Custom(Color::srgb(0.0, 0.15, 0.3)),
                ..default()
            },
            LofiEffect::new(2.0, 50.0, Vec2::new(0.125, 10.0)),
            Tonemapping::TonyMcMapface,
            Bloom::default(),
            DebandDither::Enabled,
            PlayerCamera,
            RenderLayers::from_layers(&[0, 1]), // scene + water
            Projection::Orthographic(OrthographicProjection {
                scale: 0.25,
                ..OrthographicProjection::default_2d()
            }),
        )).id();
        commands.entity(frog_id).add_child(camera_id);
    }
}