use bevy::prelude::*;

use crate::{
    GameState,
    gameplay::{
        physics::Rigidbody,
        player::{
            frog_tongue_active,
            Frog,
            grapple::Tongue,
        },
    },
};

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_tongue
                .run_if(in_state(GameState::Game))
                .run_if(frog_tongue_active),
            update_sprite_orientation
                .run_if(in_state(GameState::Game)),
        ));
        app.add_observer(on_tongue_create);
        app.add_observer(on_tongue_destroy);
    }
}

#[derive(Event)]
pub struct TongueCreate {
    pub origin: Vec2,
    pub target: Vec2,
}

#[derive(Event)]
pub struct TongueDestroy;

fn on_tongue_create(
    event: On<TongueCreate>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut sprite: Single<&mut Sprite, With<Frog>>,
) {
    let middle = (event.origin + event.target) / 2.0;
    let direction = (event.target - event.origin).normalize_or_zero();
    let angle = direction.y.atan2(direction.x);

    let tongue = meshes.add(Rectangle::new(1.0, 2.0)); // Length will later be updated in update_tongue
    let color = materials.add(Color::srgb(0.2, 0.5, 0.0));     

    commands.spawn(
        (
            Tongue,
            Mesh2d(tongue),
            MeshMaterial2d(color),
            Transform {
                translation: middle.extend(0.0),
                rotation: Quat::from_rotation_z(angle),
                ..default()
            },
        )
    );
}

fn on_tongue_destroy(
    _event: On<TongueDestroy>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, With<Tongue>>,
    mut commands: Commands,
    mut sprite: Single<&mut Sprite, With<Frog>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn update_tongue(
    mut tongue_transform: Single<&mut Transform, With<Tongue>>,
    query: Single<(&Frog, &Transform), Without<Tongue>>,
) {
    let (frog, transform) = *query;

    let distance = transform
        .translation
        .truncate()
        .distance(frog.tongue_pos);

    let target_transform = Transform::from_xyz(
            frog.tongue_pos.x,
            frog.tongue_pos.y,
            0.0);

    let middle = (transform.translation + target_transform.translation) / 2.0;
    let direction = target_transform.translation - transform.translation;
    let angle = direction.y.atan2(direction.x);

    tongue_transform.scale.x = distance;
    tongue_transform.translation = middle;
    tongue_transform.rotation = Quat::from_rotation_z(angle);
}

fn update_sprite_orientation(query: Single<(&mut Sprite, &Rigidbody), With<Frog>>) {
    let (mut sprite, rb) = query.into_inner();

    if rb.velocity.x >= 0.0 {
        sprite.flip_x = false;
    } else {
        sprite.flip_x = true;
    }
}