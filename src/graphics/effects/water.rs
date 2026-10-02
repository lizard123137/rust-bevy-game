use bevy::{
    asset::RenderAssetUsages,
    camera::{
        visibility::RenderLayers,
        RenderTarget,
    },
    core_pipeline::tonemapping::{DebandDither, Tonemapping},
    reflect::TypePath,
    post_process::bloom::Bloom,
    prelude::*,
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
};

use crate::GameState;
use crate::gameplay::player::PlayerCamera;

pub struct WaterPlugin;

impl Plugin for WaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<WaterMaterial>::default());
        app.add_systems(OnEnter(GameState::Game), (
            spawn_water,
        ));
        app.add_systems(Update, (
            update_fx_camera
                .run_if(in_state(GameState::Game)),
        ));
    }
}

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

    // Camera that renders the scene (layer 0 only) into that image
    commands.spawn((
        Camera2d,
        RenderTarget::Image(scene_image.clone().into()), // 0.15/0.16 syntax
        Camera {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            order: -1, // render before the main camera
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom::default(),
        DebandDither::Enabled,
        RenderLayers::layer(0),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.25,
            ..OrthographicProjection::default_2d()
        }),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 100.0))),
        MeshMaterial2d(materials.add(WaterMaterial {
            color: Srgba::rgb(0.0, 0.125, 0.25).into(),
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