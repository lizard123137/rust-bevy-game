use bevy::{
    color::palettes::css::*,
    math::Isometry2d,
    prelude::*,
};

use crate::player::{
    Frog,
    frog_tongue_active,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate,
            (
                apply_gravity,
                apply_tongue_forces.run_if(frog_tongue_active),
                apply_force,
                apply_velocity,

                debug_gizmos,
            ).chain(),
        );
    }
}

#[derive(Debug, Component)]
pub struct Rigidbody {
    pub mass: f32,

    pub velocity: Vec2,
    pub force: Vec2,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            mass: 1.0,

            velocity: Vec2::ZERO,
            force: Vec2::ZERO,
        }
    }
}

fn apply_force(
    time: Res<Time>,
    mut query: Query<&mut Rigidbody>,
) {
    let delta = time.delta_secs();

    for mut rb in &mut query {
        if rb.mass <= 0.0 {
            continue;
        }

        let acceleration = rb.force / rb.mass;
        
        rb.velocity += acceleration * delta;
        rb.velocity *= 0.99; // Prevent infinite speed
        rb.force = Vec2::ZERO;
    }
}

fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Rigidbody)>,
) {
    let delta = time.delta_secs();

    for (mut transform, rb) in &mut query {
        transform.translation += (rb.velocity * delta).extend(0.0);
    }
}

fn apply_gravity(mut query: Query<(&mut Rigidbody, &Transform)>) {
    for (mut rb, t) in &mut query {
        // Hack solution
        if t.translation.y < 0.0 {
            rb.velocity.y = 0.0;
            continue;
        }

        rb.force += Vec2::new(0.0, -100.0);
    }
}

fn apply_tongue_forces(
    mut query: Query<(&mut Rigidbody, &mut Transform, &Frog)>,
) {
    for (mut rb, mut transform, frog) in &mut query {
        let current_pos = transform.translation.truncate();
        let offset = current_pos - frog.target_pos;
        let current_dist = offset.length();

        if current_dist < frog.tongue_len {
            continue;
        }

        let dir = offset.normalize_or_zero();

        let radial_force = rb.force.dot(dir);
        if radial_force > 0.0 {
            rb.force -= dir * radial_force;
        }

        let radial_velocity = rb.velocity.dot(dir);
        if radial_velocity > 0.0 {
            rb.velocity -= dir * radial_velocity;
        }

        let clamped_pos = frog.target_pos + dir * frog.tongue_len;
        transform.translation.x = clamped_pos.x;
        transform.translation.y = clamped_pos.y;
    }
}

fn debug_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Rigidbody)>
) {
    gizmos.rect_2d(Isometry2d::IDENTITY, Vec2::splat(50.0), PINK);

    // Draw velocity
    for (transform, rb) in query {
        gizmos.arrow_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + rb.velocity * 5.0,
            BLUE,
        );    
    }
}