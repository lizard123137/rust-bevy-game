use avian3d::prelude::*;
use bevy::prelude::*;

pub struct PlanePlugin;

impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_lift,
                // apply_drag,
                apply_steering,
                apply_thrust,
            )
        );
    }
}

//
// Plane Physics
//

pub const AILERON_MIN: f32 = -15.0;
pub const AILERON_MAX: f32 = 15.0;
pub const ELEVATOR_MIN: f32 = -15.0;
pub const ELEVATOR_MAX: f32 = 15.0;
pub const RUDDER_MIN: f32 = -15.0;
pub const RUDDER_MAX: f32 = 15.0;

#[derive(Debug, Component)]
pub struct Plane {
    pub aileron: f32,
    pub elevator: f32,
    pub flaps: f32,
    pub rudder: f32,

    pub thrust: f32,
    pub throttle: f32,
}

impl Default for Plane {
    fn default() -> Self {
        Self {
            aileron: 0.0,
            elevator: 0.0,
            flaps: 0.0,
            rudder: 0.0,
            
            thrust: 10.0,
            throttle: 0.0,
        }
    }
}

fn apply_thrust(mut query: Query<(&Plane, &Transform, Forces)>) {
    for (plane, transform, mut forces) in &mut query {
        let forward_dir = transform.forward();
        let thrust = plane.thrust * plane.throttle;

        forces.apply_force(forward_dir * thrust);
    }
}

fn apply_lift(mut query: Query<(&Plane, &Transform, Forces)>) {
    for (plane, transform, mut forces) in &mut query {
        let forward_dir = transform.forward();
        let velocity = forces.linear_velocity();
        let forward_speed = velocity.dot(*forward_dir).max(0.0);

        let lift_force = 0.1 * plane.elevator * forward_speed * forward_speed;

        forces.apply_force(*transform.up() * lift_force);
    }
}

#[derive(Debug, Component)]
pub struct Drag {
    pub front: f32,
    pub back: f32,
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            front: 1.0,
            back: 2.0,
            left: 2.0,
            right: 2.0,
            top: 2.0,
            bottom: 2.0,
        }
    }
}

fn scale_6(
    value: Vec3,
    pos_x: f32,
    neg_x: f32,
    pos_y: f32,
    neg_y: f32,
    pos_z: f32,
    neg_z: f32
) -> Vec3 {
    let mut result = value.clone();
    
    if result.x > 0.0 {
        result.x *= pos_x;
    } else if result.x < 0.0 {
        result.x *= neg_x;
    }

    if result.y > 0.0 {
        result.y *= pos_y;
    } else if result.y < 0.0 {
        result.y *= neg_y;
    }
    
    if result.z > 0.0 {
        result.z *= pos_z;
    } else if result.z < 0.0 {
        result.z *= neg_z;
    }

    return result;
}

fn apply_steering(mut query: Query<(&Plane, &Transform, Forces)>) {
    for (plane, transform, mut forces) in &mut query {
        let torque = Vec3::new(
            plane.elevator * 0.5,
            -plane.rudder * 0.5,
            -plane.aileron * 0.5
        );

        forces.apply_local_torque(torque);
    }
}

// fn apply_drag(
//     fixed_time: Res<Time<Fixed>>,
//     mut query: Query<(&mut Velocity, &Transform, &Drag)>,
// ) {
//     let delta = fixed_time.delta_secs();

//     const DRAG_THRESHOLD: f32 = 50.0;

//     for (mut velocity, transform, drag) in query.iter_mut() {
//         let lv = transform.rotation.inverse() * velocity.0;
//         let lv2 = lv.length_squared();

//         if lv2 < 0.0001 {
//             continue;
//         }

//         let coef = scale_6(
//             lv.normalize(),
//             drag.right * (lv.x.abs() / 50.0).clamp(0.0, 1.0),
//             drag.left * (lv.x.abs() / 50.0).clamp(0.0, 1.0),
//             drag.top * (lv.y.abs() / 50.0).clamp(0.0, 1.0),
//             drag.bottom * (lv.y.abs() / 50.0).clamp(0.0, 1.0),
//             drag.front * (lv.z.abs() / 50.0).clamp(0.0, 1.0),
//             drag.back * (lv.z.abs() / 50.0).clamp(0.0, 1.0),
//         );

//         velocity.0 += coef.length() * lv2 * -lv.normalize() * delta;
//     }
// }
