pub mod physics;
pub mod player;
pub mod spatial_hash_grid;

use bevy::prelude::*;
use crate::{
    asset_loader::GameAssets,
    level::level_loader,
    GameState,
};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            physics::PhysicsPlugin,
            player::PlayerPlugin,
            spatial_hash_grid::SpatialHashGridPlugin,
        ));
        app.add_systems(OnEnter(GameState::Game),
            level_loader::spawn_level
        );
    }
}