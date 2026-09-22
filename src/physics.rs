use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_gravity,
                apply_lift,
                apply_drag,
                apply_thrust,
                apply_velocities,
            )
        );
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

#[derive(Component, Default, Debug)]
pub struct Gravity;

#[derive(Component)]
pub struct GravityScale(pub f32);

impl Default for GravityScale {
    fn default() -> Self {
        Self(1.0)
    }
}

const DEFAULT_GRAVITY: f32 = 9.81;

fn apply_gravity(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Velocity, Option<&GravityScale>), With<Gravity>>,
) {
    let delta = fixed_time.delta_secs();

    for (mut velocity, gravity_scale) in query.iter_mut() {
        let scale = gravity_scale.map(|s| s.0).unwrap_or(1.0);
        velocity.y -= DEFAULT_GRAVITY * scale * delta;
    }
}



#[derive(Component, Debug)]
pub struct Lift(pub f32);

fn apply_lift(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Velocity, &Transform, &Lift)>,
) {
    let delta = fixed_time.delta_secs();

    for (mut velocity, transform, lift) in query.iter_mut() {
        let forward_dir = transform.forward();
        let forward_speed = velocity.0.dot(*forward_dir).max(0.0);

        let lift_force = lift.0 * forward_speed * forward_speed;

        velocity.0 += *transform.up() * lift_force * delta;
    }
}

fn apply_velocities(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &Velocity,
    )>,
) {
    let delta = fixed_time.delta_secs();

    for (mut current_physical_translation, mut previous_physical_translation, velocity) in query.iter_mut() {
        previous_physical_translation.0 = current_physical_translation.0;
        current_physical_translation.0 += velocity.0 * delta;
    }
}

//
// Plane Physics
//

#[derive(Debug, Component)]
pub struct Plane {
    pub thrust: f32,
    pub throttle: f32,
}

fn apply_thrust(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Velocity, &Transform, &Plane)>,
) {
    let delta = fixed_time.delta_secs();

    for (mut velocity, transform, plane) in query.iter_mut() {
        let forward_dir = transform.forward();
        let thrust = plane.thrust * plane.throttle;

        velocity.0 += forward_dir * thrust * delta;
    }
}

#[derive(Component)]
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

fn calc_drag_coef(
    value: Vec3,
    posX: f32,
    negX: f32,
    posY: f32,
    negY: f32,
    posZ: f32,
    negZ: f32
) -> Vec3 {
    let mut result = value.clone();
    
    if result.x > 0.0 {
        result.x *= posX;
    } else if result.x < 0.0 {
        result.x *= negX;
    }

    if result.y > 0.0 {
        result.y *= posY;
    } else if result.y < 0.0 {
        result.y *= negY;
    }
    
    if result.z > 0.0 {
        result.z *= posZ;
    } else if result.z < 0.0 {
        result.z *= negZ;
    }

    return result;
}

fn apply_drag(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(&mut Velocity, &Transform, &Drag)>,
) {
    let delta = fixed_time.delta_secs();

    const DRAG_THRESHOLD: f32 = 50.0;

    for (mut velocity, transform, drag) in query.iter_mut() {
        let lv = transform.rotation.inverse() * velocity.0;
        let lv2 = lv.length_squared();

        if lv2 < 0.0001 {
            continue;
        }

        let coef = calc_drag_coef(
            lv.normalize(),
            drag.right * (lv.x.abs() / 50.0).clamp(0.0, 1.0),
            drag.left * (lv.x.abs() / 50.0).clamp(0.0, 1.0),
            drag.top * (lv.y.abs() / 50.0).clamp(0.0, 1.0),
            drag.bottom * (lv.y.abs() / 50.0).clamp(0.0, 1.0),
            drag.front * (lv.z.abs() / 50.0).clamp(0.0, 1.0),
            drag.back * (lv.z.abs() / 50.0).clamp(0.0, 1.0),
        );

        velocity.0 += coef.length() * lv2 * -lv.normalize() * delta;
    }
}
