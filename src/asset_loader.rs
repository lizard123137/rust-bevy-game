use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::{
    level::{
        level_loader::LevelData,
        dialogue::DialogueRegistry,
        tile::TileRegistry,
        item::ItemRegistry,
    },
};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<LevelData>::new(&["level.ron"]),
            RonAssetPlugin::<DialogueRegistry>::new(&["dialogues.ron"]),
            RonAssetPlugin::<TileRegistry>::new(&["tiles.ron"]),
            RonAssetPlugin::<ItemRegistry>::new(&["items.ron"]),
        ));
        app.add_systems(Startup, load_assets);
    }
}

#[derive(Resource)]
pub struct GameAssets {
    pub level: Handle<LevelData>,
    pub dialogues: Handle<DialogueRegistry>,
    pub tiles: Handle<TileRegistry>,
    pub items: Handle<ItemRegistry>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        level: asset_server.load("levels/level_1.level.ron"),
        dialogues: asset_server.load("images/dialogues/default.dialogues.ron"),
        tiles: asset_server.load("images/tiles/default.tiles.ron"),
        items: asset_server.load("images/items/default.items.ron"),
    })
}