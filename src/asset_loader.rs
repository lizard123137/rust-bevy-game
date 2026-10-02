use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    level::{
        level_loader::LevelData,
        tile::TileRegistry,
    },
};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<LevelData>::new(&["level.ron"]),
            RonAssetPlugin::<TileRegistry>::new(&["tile.ron"]),
        ));
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub level: Handle<LevelData>,
    pub tiles: Handle<TileRegistry>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        level: asset_server.load("levels/level_1.level.ron"),
        tiles: asset_server.load("images/tiles/default.tiles.ron"),
    })
}