mod level_loader;
mod tiles;

use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::gameplay::{
    player::Frog,
    physics::Rigidbody,
};
use level_loader::{LevelData, ObjectType};
use tiles::TileRegistry;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            RonAssetPlugin::<LevelData>::new(&["level.ron"]),
            RonAssetPlugin::<TileRegistry>::new(&["tile.ron"]),
        ));
        app.add_systems(Startup, (
            load_assets,
            spawn_camera,
        ));
        app.add_systems(Update, (
            spawn_level.run_if(resource_exists::<GameAssets>),
        ));
    }
}

#[derive(Resource)]
struct GameAssets {
    level: Handle<LevelData>,
    tiles: Handle<TileRegistry>,
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

fn spawn_level(
    assets: Res<GameAssets>,
    levels: Res<Assets<LevelData>>,
    registries: Res<Assets<TileRegistry>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut done: Local<bool>
) {
    if *done { return; }
    let (Some(level), Some(registry)) = (
        levels.get(&assets.level),
        registries.get(&assets.tiles),
    ) else { return };

    for obj in &level.objects {
        match &obj.obj_type {
            ObjectType::Tile {name, solid } => {
                let Some(def) = registry.tiles.get(name) else {
                    warn!("level references unknown tile '{name}'");
                    continue;
                };
                let mut tile = commands.spawn((
                    Sprite::from_image(asset_server.load(&def.sprite)),
                    Transform::from_xyz(obj.pos.x, obj.pos.y, 0.0),
                ));
                if *solid {
                    tile.insert(Rigidbody {
                        size: Vec2::new(50.0, 50.0),
                        moveable: false,
                        ..default()
                    });
                }
            }
            ObjectType::Mushroom => {
                commands.spawn((
                    Sprite::from_image(
                    asset_server
                        .load("images/mushroom_red.png")
                    ),
                    Transform::from_xyz(obj.pos.x, obj.pos.y, 1.0),
                ));
            }
            ObjectType::Player => {
                commands.spawn((
                    Rigidbody::default(),
                    Frog {
                        grounded: false,
                        tongue_active: false,
                        target_pos: Vec2::new(0.0, 0.0),
                        tongue_len: 0.0,
                        range: 50.0,
                    },
                    Sprite::from_image(asset_server.load("images/frog.png")),
                    Transform::from_xyz(obj.pos.x, obj.pos.y, 0.0),
                ));
            }
        }
    }
    *done = true;
}

fn spawn_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..default()
        },
    ));
}