use bevy::prelude::*;

mod hud;
mod player;
mod physics;

use hud::HUDPlugin;
use player::PlayerPlugin;
use physics::PhysicsPlugin;

// 2D magic frog game where you swing by tongue
// The frog can jump left and right with you holding the arrows for a specified time
// The jump strength increses as you hold it

// For FX add vignette, screen shake and pixelated bloom

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HUDPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, (
            spawn_world,
        ))
        .run();
}

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {

    let floor = meshes.add(Rectangle::new(500.0, 10.0));
    let color = materials.add(Color::srgb(0.0, 0.7, 0.2));     
    
    commands.spawn(
        (
            Mesh2d(floor),
            MeshMaterial2d(color),
            Transform::from_xyz(0.0, 0.0, 0.0),
        )
    );
}