use crate::physics::{
    Velocity,
    PhysicalTranslation,
    PreviousPhysicalTranslation,
    Plane,
    Gravity,
    GravityScale,
    Lift,
    Drag,
};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DidFixedTimestepRunThisFrame>();
        app.add_systems(PreUpdate, clear_fixed_timestep_flag);
        app.add_systems(FixedPreUpdate, set_fixed_time_step_flag);
        app.add_systems(
            Startup,
            (
                spawn_player,
            ),
        );
        app.add_systems(
            RunFixedMainLoop,
            (
                (
                    accumulate_input
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
                (
                    clear_input.run_if(did_fixed_timestep_run_this_frame),
                    interpolate_rendered_transform,
                )
                    .chain()
                    .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
            ),
        );
    }
}

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput {
    rotation: Vec3,
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let spawn_position = Vec3::new(0.0, 5.0, 0.0);

    commands.spawn((
        Name::new("Player"),
        Transform::from_translation(spawn_position),
        AccumulatedInput::default(),
        Velocity::default(),
        PhysicalTranslation(spawn_position),
        PreviousPhysicalTranslation(spawn_position),
        Gravity,
        GravityScale(0.5),
        Lift(0.5),
        Plane{thrust: 10.0, throttle: 0.5},
        Drag::default(),
        children![
            (
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
                Transform::from_xyz(0.0, 2.0, 3.0),
            ),
            (
                Mesh3d(
                    asset_server.load("models/plane.obj")
                ),
                MeshMaterial3d(materials.add(StandardMaterial {
                    base_color: Color::linear_rgb(1.0, 0.5, 0.0),
                    ..default()
                })),
                Transform::from_rotation(
                    Quat::from_euler(
                        EulerRot::XYZ,
                        -90.0_f32.to_radians(),
                        0.0_f32.to_radians(),
                        90.0_f32.to_radians(),
                    )
                ),
            )
        ],
    ));
}

fn accumulate_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(
        &mut Transform,
        &mut AccumulatedInput,
        &mut Plane,
        &Velocity
    )>,
) {
    let (
        mut player_transform,
        mut input,
        mut plane,
        velocity
    ) = player.into_inner();
    input.rotation = Vec3::ZERO;
    
    const TURN_SPEED: f32 = 0.1;
    const TURN_SPEED_AT: f32 = 50.0;

    let forward_speed = velocity.0
        .dot(*player_transform.forward())
        .max(0.0);

    let speed_factor = (forward_speed / TURN_SPEED_AT).clamp(0.0, 1.0);
    let turn_amount = TURN_SPEED * speed_factor;

    if keyboard_input.pressed(KeyCode::KeyW) {
        input.rotation.x -= turn_amount;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        input.rotation.x += turn_amount;
    }
    if keyboard_input.pressed(KeyCode::KeyQ) {
        input.rotation.y += turn_amount;
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        input.rotation.y -= turn_amount;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        input.rotation.z += turn_amount;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        input.rotation.z -= turn_amount;
    }

    // Throttle
    if keyboard_input.pressed(KeyCode::KeyF) {
        plane.throttle += 0.1;
    }
    if keyboard_input.pressed(KeyCode::KeyV) {
        plane.throttle -= 0.1;
    }
    plane.throttle = plane.throttle.clamp(0.0, 1.0);

    let delta_rotation = Quat::from_euler(
        EulerRot::YXZ,
        input.rotation.y,
        input.rotation.x,
        input.rotation.z,
    );

    player_transform.rotation *= delta_rotation;
}

#[derive(Resource, Debug, Deref, DerefMut, Default)]
pub struct DidFixedTimestepRunThisFrame(bool);

fn clear_fixed_timestep_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = false;
}

fn set_fixed_time_step_flag(
    mut did_fixed_timestep_run_this_frame: ResMut<DidFixedTimestepRunThisFrame>,
) {
    did_fixed_timestep_run_this_frame.0 = true;
}

fn did_fixed_timestep_run_this_frame(
    did_fixed_timestep_run_this_frame: Res<DidFixedTimestepRunThisFrame>,
) -> bool {
    did_fixed_timestep_run_this_frame.0
}

fn clear_input(mut input: Single<&mut AccumulatedInput>) {
    **input = AccumulatedInput::default();
}

fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in query.iter_mut() {
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;

        let alpha = fixed_time.overstep_fraction();

        let rendered_translation = previous.lerp(current, alpha);
        transform.translation = rendered_translation;
    }
}