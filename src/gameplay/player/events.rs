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
    frog: Single<&Frog>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
) {
    if frog.tongue_active {
        sprite.image = asset_server.load("images/frog_jump_open.png");
    } else {
        sprite.image = asset_server.load("images/frog_jump.png");
    }
}

#[derive(Event)]
pub struct PlayerLand;

fn on_player_land(
    _event: On<PlayerLand>,
    asset_server: Res<AssetServer>,
    frog: Single<&Frog>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
) {
    if frog.tongue_active {
        sprite.image = asset_server.load("images/frog_open.png");
    } else {
        sprite.image = asset_server.load("images/frog.png");
    }
}