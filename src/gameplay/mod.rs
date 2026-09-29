pub mod physics;
pub mod player;
pub mod scene;

use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            physics::PhysicsPlugin,
            player::PlayerPlugin,
            scene::ScenePlugin,
        ));
    }
}