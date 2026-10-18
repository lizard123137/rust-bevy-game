use bevy::prelude::*;

use crate::{
    asset_loader::GameAssets,
    editor::{
        change_list::{
            EditorChange,
            EditorChangeList,
            EditorChangeType,
        },
        file_handler,
    },
    level::{
        level_loader::{
            ObjectData,
            ObjectType,
            Registries,
        },
        spawners,
    },
};

pub struct EditorEventsPlugin;

impl Plugin for EditorEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_editor_place);
        app.add_observer(on_editor_remove);
        app.add_observer(on_editor_save);
    }
}

#[derive(Event)]
pub struct EditorPlace {
    pub pos: Vec3,
    pub name: String,
}

fn on_editor_place(
    event: On<EditorPlace>,
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    registries: Registries,
    mut change_list: ResMut<EditorChangeList>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let Some(tile_registry) = registries.tile.get(&assets.tiles) else { return };

    spawners::spawn_tile(&mut commands, &mut texture_atlas_layout, &asset_server, &tile_registry, event.pos, &event.name);
    change_list.changes.push(
        EditorChange {
            change_type: EditorChangeType::Add,
            object: Some(ObjectData {
                obj_type: ObjectType::Tile {
                    name: event.name.clone(),
                    solid: true,
                },
                pos: event.pos,
            }),
        }
    );
}

#[derive(Event)]
pub struct EditorRemove {
    pub pos: Vec3,
}

fn on_editor_remove(
    event: On<EditorRemove>,
    mut change_list: ResMut<EditorChangeList>,
) {
    change_list.changes.push(
        EditorChange {
            change_type: EditorChangeType::Remove,
            object: None,
        }
    );
}

#[derive(Event)]
pub struct EditorSave;

fn on_editor_save(
    _event: On<EditorSave>,
    change_list: Res<EditorChangeList>,
) {
    file_handler::file_changes_apply(&change_list);
}