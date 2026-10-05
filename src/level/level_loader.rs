use serde::{Deserialize, Serialize};

use bevy::prelude::*;

use super::spawners;
use crate::{
    asset_loader::GameAssets,
    level::{
        mushroom::MushroomType,
        spawners::shader::ShaderMaterials,
        tile::TileRegistry,
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
    Mushroom {kind: MushroomType},
    Shader {name: String, size: Vec2},
    Tile {name: String, solid: bool},
    Player,
}

pub fn spawn_level(
    assets: Res<GameAssets>,
    levels: Res<Assets<LevelData>>,
    registries: Res<Assets<TileRegistry>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ShaderMaterials,
    mut done: Local<bool>
) {
    if *done { return; }
    let (Some(level), Some(registry)) = (
        levels.get(&assets.level),
        registries.get(&assets.tiles),
    ) else { return };

    for obj in &level.objects {
        match &obj.obj_type {
            ObjectType::Mushroom {kind} => {
                let mut m = commands.spawn((
                    Transform::from_xyz(obj.pos.x, obj.pos.y, 1.0),
                ));
                m.insert(
                    // TODO extract this into a ron file
                    match kind {
                        MushroomType::Blue => Sprite::from_image(asset_server.load("images/mushroom_blue.png")),
                        MushroomType::Brown => Sprite::from_image(asset_server.load("images/mushroom_brown.png")),
                        MushroomType::Red => Sprite::from_image(asset_server.load("images/mushroom_red.png")),
                    }
                );
            }
            // TODO come up with something better than passing shader type as string
            ObjectType::Shader {name, size} => spawners::spawn_shader(&mut commands, &mut meshes, &mut materials, &asset_server, obj.pos, name, *size),
            ObjectType::Tile {name, solid } => spawners::spawn_tile(&mut commands, &mut texture_atlas_layout, &asset_server, &registry, obj.pos, name, *solid),
            ObjectType::Player => spawners::spawn_player(&mut commands, &asset_server, obj.pos),
        }
    }
    *done = true;
}