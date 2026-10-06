use bevy::prelude::*;
use crate::{
    level::item::ItemRegistry,
};

pub fn spawn_item(
    commands: &mut Commands,
    asset_server: &AssetServer,
    registry: &ItemRegistry,
    pos: Vec2,
    name: &str,
) {
    let Some(def) = registry.get(name) else {
        warn!("level references unknown item '{name}'");
        return;
    };

    // TODO add rigidbody and apply effect on collision with player
    commands.spawn((
        Sprite::from_image(asset_server.load(&def.sprite)),
        Transform::from_xyz(pos.x, pos.y, 0.0),
    ));
}