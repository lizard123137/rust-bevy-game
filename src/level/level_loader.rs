use serde::{Deserialize, Serialize};

use bevy::{
    ecs::system::SystemParam,
    prelude::*,
};

use super::spawners;
use crate::{
    asset_loader::GameAssets,
    level::{
        spawners::shader::ShaderMaterials,
        dialogue::DialogueRegistry,
        tile::TileRegistry,
        item::ItemRegistry,
    },
};

#[derive(Asset, TypePath, Serialize, Deserialize, Debug, Clone)]
pub struct LevelData {
    pub name: String,
    pub description: String,
    pub objects: Vec<ObjectData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectData {
    pub obj_type: ObjectType,
    pub pos: Vec2,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ObjectType {
    Dialogue {name: String, sentences: Vec<String>},
    Item {name: String},
    Shader {name: String, size: Vec2},
    Tile {name: String, solid: bool},
    Player,
}

#[derive(SystemParam)]
pub struct Registries<'w> {
    pub dialogue: Res<'w, Assets<DialogueRegistry>>,
    pub tile: Res<'w, Assets<TileRegistry>>,
    pub item: Res<'w, Assets<ItemRegistry>>,
}

pub fn spawn_level(
    assets: Res<GameAssets>,
    levels: Res<Assets<LevelData>>,
    asset_server: Res<AssetServer>,
    registries: Registries,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ShaderMaterials,
    mut done: Local<bool>
) {
    if *done { return; }
    let (
        Some(level),
        Some(dialogue_registry),
        Some(tile_registry),
        Some(item_registry)
    ) = (
        levels.get(&assets.level),
        registries.dialogue.get(&assets.dialogues),
        registries.tile.get(&assets.tiles),
        registries.item.get(&assets.items),
    ) else { return };

    for obj in &level.objects {
        match &obj.obj_type {
            ObjectType::Dialogue {name, sentences} => spawners::spawn_dialogue(&mut commands, &asset_server, &dialogue_registry, obj.pos, name, &sentences),
            // TODO come up with something better than passing shader type as string
            ObjectType::Shader {name, size} => spawners::spawn_shader(&mut commands, &mut meshes, &mut materials, obj.pos, name, *size),
            ObjectType::Tile {name, solid } => spawners::spawn_tile(&mut commands, &mut texture_atlas_layout, &asset_server, &tile_registry, obj.pos, name, *solid),
            ObjectType::Player => spawners::spawn_player(&mut commands, &asset_server, obj.pos),
            ObjectType::Item {name} => spawners::spawn_item(&mut commands, &asset_server, &item_registry, obj.pos, name),
        }
    }
    *done = true;
}