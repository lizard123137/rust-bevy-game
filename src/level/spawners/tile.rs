use bevy::prelude::*;
use std::collections::HashSet;                
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
) {
    let Some(def) = registry.get(name) else {
        warn!("level references unknown tile '{name}'");
        return;
    };

    match &def.animated {
        true => {
            let texture = asset_server.load(&def.sprite);
            let layout = TextureAtlasLayout::from_grid(UVec2::splat(16), def.frames as u32, 1, None, None);
            let texture_atlas_layout = texture_atlas_layout.add(layout);
            let animation_indices = AnimationIndices { first: 0, last: &def.frames - 1 };

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
}

pub fn spawn_tile_colliders(
    commands: &mut Commands,
    cells: &HashSet<IVec2>,
) {
    let mut remaining = cells.clone();

    let mut order: Vec<IVec2> = cells.iter().copied().collect();
    order.sort_by_key(|c| (c.y, c.x));

    for start in order {
        if !remaining.contains(&start) {
            continue;
        }

        let mut width = 1;
        while remaining.contains(&(start + IVec2::new(width, 0))) {
            width += 1;
        }

        let mut height = 1;
        'grow: loop {
            for x in 0..width {
                if !remaining.contains(&(start + IVec2::new(x, height))) {
                    break 'grow;
                }
            }
            height += 1;
        }

        for y in 0..height {
            for x in 0..width {
                remaining.remove(&(start + IVec2::new(x, y)));
            }
        }

        let cells_size = Vec2::new(width as f32, height as f32);
        let center = (start.as_vec2() + (cells_size - 1.0) / 2.0) * 16.0;
        let size = cells_size * 16.0 - 2.0;

        commands.spawn((
            Transform::from_xyz(center.x, center.y, 0.0),
            Rigidbody {
                size: size,
                moveable: false,
                ..default()
            },
        ));
    }
}