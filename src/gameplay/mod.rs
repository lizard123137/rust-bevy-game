pub mod physics;
pub mod player;

use bevy::prelude::*;
use crate::GameState;
use crate::asset_loader::GameAssets;
use crate::level::level_loader;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            physics::PhysicsPlugin,
            player::PlayerPlugin,
        ));
        app.add_systems(Update,
            level_loader::spawn_level
                .run_if(in_state(GameState::Game))
                .run_if(resource_exists::<GameAssets>),
        );
    }
}