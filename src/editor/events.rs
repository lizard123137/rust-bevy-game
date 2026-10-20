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
        state::EditorState,
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
        app.add_observer(on_editor_layer_select);
    }
}

#[derive(Event)]
pub struct EditorPlace {
    pub pos: Vec3,
}

fn on_editor_place(
    event: On<EditorPlace>,
    assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
    registries: Registries,
    state: Res<EditorState>,
    mut change_list: ResMut<EditorChangeList>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let Some(tile_registry) = registries.tile.get(&assets.tiles) else { return };

    match &state.selected_object {
        ObjectType::Tile {name, solid } => {
            spawners::spawn_tile(&mut commands, &mut texture_atlas_layout, &asset_server, &tile_registry, event.pos, &name);
        },
        _ => todo!(),
    }
    
    change_list.changes.push(
        EditorChange {
            change_type: EditorChangeType::Add,
            object: ObjectData {
                obj_type: state.selected_object.clone(),
                pos: event.pos,
            },
        }
    );
}

#[derive(Event)]
pub struct EditorRemove {
    pub pos: Vec3,
}

fn on_editor_remove(
    event: On<EditorRemove>,
    query: Query<(Entity, &Transform)>,
    mut change_list: ResMut<EditorChangeList>,
    mut commands: Commands,
) {
    let to_remove: Vec<Entity> = query.iter()
        .find_map(|(entity, transform)| {
            let entity_position = transform.translation;
            (entity_position == event.pos).then_some(entity)
        })
        .into_iter()
        .collect();

    for entity in to_remove {
        commands.entity(entity).despawn();
    }

    change_list.changes.push(
        EditorChange {
            change_type: EditorChangeType::Remove,
            object: ObjectData {
                pos: event.pos,
                obj_type: ObjectType::Empty,
            },
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

#[derive(Event)]
pub struct EditorLayerSelect {
    pub layer: f32,
}

fn on_editor_layer_select(
    event: On<EditorLayerSelect>,
    mut state: ResMut<EditorState>,
) {
    state.selected_layer = event.layer;
}