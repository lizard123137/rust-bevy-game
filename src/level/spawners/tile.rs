use bevy::prelude::*;
use crate::{
    gameplay::physics::Rigidbody,
    graphics::animation::{AnimationIndices, AnimationTimer},
    level::tile::TileRegistry,
};

pub fn spawn_tile(
    commands: &mut Commands,
    texture_atlas_layout: &mut Assets<TextureAtlasLayout>,
    asset_server: &AssetServer,
    registry: &TileRegistry,
    pos: Vec2,
    name: &str,
    solid: bool
) {
    let Some(def) = registry.tiles.get(name) else {
        warn!("level references unknown tile '{name}'");
        return;
    };

    let mut tile = match &def.animated {
        true => {
            let texture = asset_server.load(&def.sprite);
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: 3 };

            // TODO right now it creates a separate animation for all occurences
            // TODO cache the animation and reuse it
            commands.spawn((
                Sprite::from_atlas_image(
                    texture,
                    TextureAtlas {
                        layout: texture_atlas_layout,
                        index: animation_indices.first,
                    },
                ),
                Transform::from_xyz(pos.x, pos.y, 0.0),
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating))
            ))
        },
        false => {
            commands.spawn((
                Sprite::from_image(asset_server.load(&def.sprite)),
                Transform::from_xyz(pos.x, pos.y, 0.0),
            ))
        }
    };
                
    if solid {
        tile.insert(Rigidbody {
            size: Vec2::new(14.0, 14.0),
            moveable: false,
            ..default()
        });
    }
}