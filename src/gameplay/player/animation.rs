use bevy::prelude::*;

use crate::{
    GameState,
    gameplay::{
        physics::Rigidbody,
        player::Frog,
    },
};

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_sprite_orientation
                .run_if(in_state(GameState::Game)),
        ));
    }
}

fn update_sprite_orientation(query: Single<(&mut Sprite, &Rigidbody), With<Frog>>) {
    let (mut sprite, rb) = query.into_inner();

    if rb.velocity.x >= 0.0 {
        sprite.flip_x = false;
    } else {
        sprite.flip_x = true;
    }
}