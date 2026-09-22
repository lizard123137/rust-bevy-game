use crate::plane::Plane;

use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_player,
            ),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    commands.spawn((
        Name::new("Player"),
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Transform::from_xyz(0.0, 4.0, 3.0),
        Visibility::default(),
        Plane::default(),
        children![
            (
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
                Transform::from_xyz(0.0, 2.0, 3.0),
            ),
            (
                Mesh3d(
                    asset_server.load("models/plane.obj")
                ),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(1.0, 0.5, 0.0),
                    ..default()
                })),
                Transform::from_rotation(
                    Quat::from_euler(
                        EulerRot::XYZ,
                        -90.0_f32.to_radians(),
                        0.0_f32.to_radians(),
                        90.0_f32.to_radians(),
                    )
                ),
            )
        ],
    ));
}
