use bevy::prelude::*;

use crate::gameplay::player::Frog;

pub struct PlayerEventsPlugin;

impl Plugin for PlayerEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_player_jump);        
        app.add_observer(on_player_land);
    }
}

#[derive(Event)]
pub struct PlayerJump;

fn on_player_jump(
    _event: On<PlayerJump>,
    asset_server: Res<AssetServer>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
) {
    sprite.image = asset_server.load("images/frog_jump_open.png");
}

#[derive(Event)]
pub struct PlayerLand;

fn on_player_land(
    _event: On<PlayerLand>,
    asset_server: Res<AssetServer>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
) {
    sprite.image = asset_server.load("images/frog.png");
}