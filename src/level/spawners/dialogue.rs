use bevy::prelude::*;
use crate::{
    level::dialogue::DialogueRegistry,
};

pub fn spawn_dialogue(
    commands: &mut Commands,
    asset_server: &AssetServer,
    registry: &DialogueRegistry,
    pos: Vec2,
    name: &str,
    sentences: &Vec<String>
) {
    let Some(def) = registry.get(name) else {
        warn!("level references unknown dialogue '{name}'");
        return;
    };

    for s in sentences {
        warn!("{}: {}", name, s);
    }

    commands.spawn((
        Sprite {
            image: asset_server.load(&def.sprite),
            color: Color::srgb(3.0, 1.5, 3.0),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}