use bevy::{
    color::palettes::css::*,
    math::bounding::{Aabb2d, BoundingVolume, IntersectsVolume},
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
                check_collisions,
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
    pub size: Vec2,

    pub moveable: bool,
    pub velocity: Vec2,
    pub force: Vec2,
}

impl Default for Rigidbody {
    fn default() -> Self {
        Self {
            mass: 1.0,
            size: Vec2::new(20.0, 20.0),

            moveable: true,
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
        if !rb.moveable {
            continue;
        }

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
        if !rb.moveable {
            continue;
        }

        transform.translation += (rb.velocity * delta).extend(0.0);
    }
}

fn apply_gravity(mut query: Query<&mut Rigidbody>) {
    for mut rb in &mut query {
        if !rb.moveable {
            continue;
        }

        rb.force += Vec2::new(0.0, -100.0);
    }
}

fn apply_tongue_forces(
    mut query: Query<(&mut Rigidbody, &mut Transform, &Frog)>,
) {
    for (mut rb, mut transform, frog) in &mut query {
        if !rb.moveable {
            continue;
        }

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

fn check_collisions(mut query: Query<(Entity, &mut Transform, &mut Rigidbody)>) {
    let mut combinations = query.iter_combinations_mut();

    while let Some([(ent_a, mut t_a, mut rb_a), (ent_b, mut t_b, mut rb_b)]) = combinations.fetch_next() {
        if !rb_a.moveable && !rb_b.moveable {
            continue;
        }

        let aabb_a = Aabb2d::new(t_a.translation.truncate(), rb_a.size / 2.0);
        let aabb_b = Aabb2d::new(t_b.translation.truncate(), rb_b.size / 2.0);

        if aabb_a.intersects(&aabb_b) {
            let a_hs = aabb_a.half_size();
            let a_ctr = aabb_a.center();
            let b_hs = aabb_b.half_size();
            let b_ctr = aabb_b.center();

            let overlap_x = (a_hs.x + b_hs.x) - (a_ctr.x - b_ctr.x).abs();
            let overlap_y = (a_hs.y + b_hs.y) - (a_ctr.y - b_ctr.y).abs();

            if overlap_x < overlap_y {
                let sign = if a_ctr.x < b_ctr.x { -1.0 } else { 1.0 };
                let push = Vec3::new(overlap_x * sign, 0.0, 0.0);

                if rb_a.moveable && !rb_b.moveable {
                    t_a.translation += push;
                    rb_a.velocity.x = 0.0
                } else if !rb_a.moveable && rb_b.moveable {
                    t_b.translation -= push;
                    rb_b.velocity.x = 0.0;
                } else {
                    t_a.translation += push * 0.5;
                    t_b.translation -= push * 0.5;
                    rb_a.velocity.x = 0.0;
                    rb_b.velocity.x = 0.0;
                }
            } else {
                let sign = if a_ctr.y < b_ctr.y { -1.0 } else { 1.0 };
                let push = Vec3::new(0.0, overlap_y * sign, 0.0);
                
                if rb_a.moveable && !rb_b.moveable {
                    t_a.translation += push;
                    rb_a.velocity.y = 0.0
                } else if !rb_a.moveable && rb_b.moveable {
                    t_b.translation -= push;
                    rb_b.velocity.y = 0.0;
                } else {
                    t_a.translation += push * 0.5;
                    t_b.translation -= push * 0.5;
                    rb_a.velocity.y = 0.0;
                    rb_b.velocity.y = 0.0;
                }
            }

        }
    }
}

fn debug_gizmos(
    mut gizmos: Gizmos,
    query: Query<(&Transform, &Rigidbody)>
) {

    for (transform, rb) in query {
        // Draw collider
        if rb.moveable {
            gizmos.rect_2d(transform.translation.truncate(), rb.size, PINK);
        } else {
            gizmos.rect_2d(transform.translation.truncate(), rb.size, RED);
        }

        // Draw velocity
        gizmos.arrow_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + rb.velocity * 5.0,
            BLUE,
        );
    }
}