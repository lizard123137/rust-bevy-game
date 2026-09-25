use bevy::{
    asset::RenderAssetUsages,
    camera::{
        visibility::RenderLayers,
        RenderTarget,
    },
    reflect::TypePath,
    render::{
        render_resource::{
            AsBindGroup,
            Extent3d,
            TextureDimension,
            TextureFormat,
            TextureUsages,
        },
    },
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
    prelude::*
};

mod hud;
mod player;
mod physics;

use hud::HUDPlugin;
use player::{PlayerPlugin, PlayerCamera};
use physics::{Rigidbody, PhysicsPlugin};

// 2D magic frog game where you swing by tongue
// The frog can jump left and right with you holding the arrows for a specified time
// The jump strength increses as you hold it

// For FX add vignette, screen shake and pixelated bloom

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Material2dPlugin::<WaterMaterial>::default())
        .add_plugins(HUDPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(PhysicsPlugin)
        .add_systems(Startup, (
            spawn_world,
            spawn_water,
        ))
        .add_systems(Update, (
            update_fx_camera,
        ))
        .run();
}

// Water shader

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct WaterMaterial {
    #[uniform(0)]
    color: LinearRgba,

    #[texture(1)]
    #[sampler(2)]
    scene_texture: Handle<Image>,

    #[uniform(3)]
    surface_y: f32,
}

impl Material2d for WaterMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn spawn_water(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
) {
    let size = Extent3d { width: 1280, height: 720, depth_or_array_layers: 1 };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING
        | TextureUsages::COPY_DST
        | TextureUsages::RENDER_ATTACHMENT;
    let scene_image = images.add(image);

    // 2. Camera that renders the scene (layer 0 only) into that image
    commands.spawn((
        Camera2d,
        RenderTarget::Image(scene_image.clone().into()), // 0.15/0.16 syntax
        Camera {
            order: -1, // render before the main camera
            ..default()
        },
        Transform::from_scale(Vec3::splat(0.25)), // TODO zoom with scroll
        RenderLayers::layer(0),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 100.0))),
        MeshMaterial2d(materials.add(WaterMaterial {
            color: LinearRgba::BLUE,
            scene_texture: scene_image,
            surface_y: 5.0,
        })),
        Transform::from_xyz(0.0, -50.0, 0.0),
        RenderLayers::layer(1),
    ));
}

fn update_fx_camera(
    mut camera_fx: Query<&mut Transform, (With<Camera>, Without<PlayerCamera>)>,
    transform_camera_main: Single<&GlobalTransform, With<PlayerCamera>>,
) {
    for mut transform in &mut camera_fx {
        transform.translation = transform_camera_main.translation();
    };
}

// Spawning world

fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let floor = meshes.add(Rectangle::new(500.0, 10.0));
    let color = materials.add(Color::srgb(0.0, 0.7, 0.2));     

    commands.spawn((
        Rigidbody {
            size: Vec2::new(500.0, 10.0),
            moveable: false,
            ..default()
        },
        Mesh2d(floor),
        MeshMaterial2d(color),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}