use bevy::prelude::*;

use crate::gameplay::physics::Rigidbody;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_world);
    }
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Rigidbody {
            size: Vec2::new(500.0, 10.0),
            moveable: false,
            ..default()
        },
        Sprite {
            image: asset_server.load("images/grass.png"),
            image_mode: SpriteImageMode::Tiled {
                tile_x: true,
                tile_y: false,
                stretch_value: 0.5,
            },
            custom_size: Some(Vec2::new(500.0, 10.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        Sprite::from_image(
            asset_server
                .load("images/mushroom_blue.png")
        ),
        Transform::from_xyz(-50.0, 13.0, 1.0),
    ));
    commands.spawn((
        Sprite::from_image(
            asset_server
                .load("images/mushroom_brown.png")
        ),
        Transform::from_xyz(0.0, 13.0, 1.0),
    ));
    commands.spawn((
        Sprite::from_image(
            asset_server
                .load("images/mushroom_red.png")
        ),
        Transform::from_xyz(50.0, 13.0, 1.0),
    ));
    
    commands.spawn((
        Sprite {
            image: asset_server.load("images/druid.png"),
            color: Color::srgb(3.5, 2.0, 3.5),
            ..default()
        },
        Transform::from_xyz(100.0, 13.0, -1.0),
    ));
}