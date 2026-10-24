mod animation;
pub mod events;
mod grapple;
mod jump;

use animation::PlayerAnimationPlugin;
use events::PlayerEventsPlugin;
use grapple::PlayerGrapplePlugin;
use jump::PlayerJumpPlugin;

use bevy::prelude::*;

pub struct PlayerPlugin;

pub fn frog_tongue_active(query: Query<&Frog>) -> bool {
    query.single().map_or(false, |frog| frog.tongue_active)
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerAnimationPlugin,
            PlayerEventsPlugin,
            PlayerGrapplePlugin,
            PlayerJumpPlugin,
        ));
    }
}

#[derive(Debug, Component)]
pub struct PlayerCamera;

#[derive(Debug, Component)]
pub struct Frog {
    pub grounded: bool,
    pub tongue_active: bool,

    pub target_pos: Vec2,

    pub tongue_pos: Vec2,
    pub tongue_len: f32,
    pub range: f32,
}