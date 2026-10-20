use bevy::{
    input::common_conditions::*,
    prelude::*,
};

use crate::{
    audio::PlaySound,
    GameState,
    gameplay::{
        physics::Rigidbody,
        player::{
            events::PlayerJump,
            Frog,
        },
    },
};

pub struct PlayerJumpPlugin;

impl Plugin for PlayerJumpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            activate_jump
                .run_if(in_state(GameState::Game))
                .run_if(input_just_pressed(MouseButton::Left)), // TODO refactor to be able to assign different buttons
        ));
    }
}

fn activate_jump(
    query: Single<(&mut Rigidbody, &Transform, &mut Frog)>,
    mut commands: Commands,
) {
    let (mut rb, t, mut frog) = query.into_inner();

    if !frog.grounded {
        return;
    }

    let mut direction = frog.target_pos - t.translation.truncate();
    let magnitude = direction.length();
    direction /= magnitude; // Normalize

    let jump_strength = magnitude.clamp(0.0, frog.range);

    rb.force += direction * jump_strength * 200.0;

    commands.trigger(PlaySound { name: "jump".to_string() });
    commands.trigger(PlayerJump);
    frog.grounded = false;
}