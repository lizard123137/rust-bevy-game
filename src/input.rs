use crate::plane::{
    Plane,
    AILERON_MIN,
    AILERON_MAX,
    ELEVATOR_MIN,
    ELEVATOR_MAX,
    RUDDER_MIN,
    RUDDER_MAX,
};

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            input_keyboard,
        ));
    }
}

fn input_keyboard(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut plane: Single<&mut Plane>,
) {
    const CHANGE_SPEED_AILERON: f32 = 1.0;
    const CHANGE_SPEED_ELEVATOR: f32 = 1.0;
    const CHANGE_SPEED_RUDDER: f32 = 1.0;
    const CHANGE_SPEED_THROTTLE: f32 = 0.1;

    // Ailerons
    if keyboard_input.pressed(KeyCode::KeyA) {
        plane.aileron -= CHANGE_SPEED_AILERON;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        plane.aileron += CHANGE_SPEED_AILERON;
    }
    plane.aileron = plane.aileron.clamp(AILERON_MIN, AILERON_MAX);

    // Elevators
    if keyboard_input.pressed(KeyCode::KeyS) {
        plane.elevator += CHANGE_SPEED_ELEVATOR;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        plane.elevator -= CHANGE_SPEED_ELEVATOR;
    }
    plane.elevator = plane.elevator.clamp(ELEVATOR_MIN, ELEVATOR_MAX);

    // Rudder
    if keyboard_input.pressed(KeyCode::KeyQ) {
        plane.rudder -= CHANGE_SPEED_RUDDER;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        plane.rudder += CHANGE_SPEED_RUDDER;
    }
    plane.rudder = plane.rudder.clamp(RUDDER_MIN, RUDDER_MAX);

    // Throttle
    if keyboard_input.pressed(KeyCode::KeyF) {
        plane.throttle += CHANGE_SPEED_THROTTLE;
    }
    if keyboard_input.pressed(KeyCode::KeyV) {
        plane.throttle -= CHANGE_SPEED_THROTTLE;
    }
    plane.throttle = plane.throttle.clamp(0.0, 1.0);
}