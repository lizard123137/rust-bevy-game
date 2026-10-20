use bevy::prelude::*;

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
) {
    // TODO change player sprite to in-air
    todo!();
}

#[derive(Event)]
pub struct PlayerLand;

fn on_player_land(
    _event: On<PlayerLand>,
) {
    // TODO change player sprite to landed
    todo!();
}