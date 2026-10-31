use bevy::prelude::*;

use crate::{
    level::dialogue::DialogueRegistry,
    gameplay::dialogue::Dialogue,
};

pub fn spawn_dialogue(
    commands: &mut Commands,
    asset_server: &AssetServer,
    registry: &DialogueRegistry,
    pos: Vec3,
    name: &str,
    sentences: &Vec<String>
) {
    let Some(def) = registry.get(name) else {
        warn!("level references unknown dialogue '{name}'");
        return;
    };

    commands.spawn((
        Dialogue {
            sentences: sentences.clone(),
            current: 0,
            timer: Timer::from_seconds(5.0, TimerMode::Once),
            active: false,
        },
        Sprite {
            image: asset_server.load(&def.sprite),
            color: Color::srgb(3.0, 1.5, 3.0),
            ..default()
        },
        Transform::from_xyz(pos.x, pos.y, pos.z),
    ));
}