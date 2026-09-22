use bevy::prelude::*;
use bevy_obj::ObjPlugin;

mod hud;
mod player;
mod physics;
mod scene;
mod terrain;

use hud::HUDPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;
use scene::ScenePlugin;
use terrain::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ObjPlugin)
        .add_plugins(HUDPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(ScenePlugin)
        .add_plugins(TerrainPlugin)
        .add_systems(Startup, (
            spawn_world,
            spawn_lights,
        ))
        .run();
}

//
// Notes on making realistic flight
//
// Drag increases quadratically with speed. Going 2x fast increases drag 4x.
// Flaps increase lift at the cost of also increasing drag. (Only used at slow speeds).
//
// TODOS
// TODO add mass
// TODO account for angle of attack for lift
//

fn spawn_world(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = meshes.add(Plane3d::new(Vec3::Y, Vec2::splat(10.0)));
    let cube = meshes.add(Cuboid::new(2.0, 0.5, 1.0));
    let material = materials.add(Color::WHITE);

    commands.spawn((Mesh3d(floor), MeshMaterial3d(material.clone())));

    commands.spawn((
        Mesh3d(cube.clone()),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.0, 0.25, -3.0),
    ));

    commands.spawn((
        Mesh3d(cube),
        MeshMaterial3d(material.clone()),
        Transform::from_xyz(0.75, 1.75, 0.0),
    ));

    commands.spawn((
        Mesh3d(
                asset_server.load("models/plane.obj")
        ),
        MeshMaterial3d(material),
        Transform::from_xyz(0.5, 5.0, 0.0),
    ));
}

fn spawn_lights(mut commands: Commands) {
    commands.spawn((
        PointLight {
            color: Color::WHITE,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(-2.0, 4.0, -0.75),
    ));
}
