use bevy::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                apply_gravity,
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