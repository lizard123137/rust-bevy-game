use bevy::{
    camera::visibility::RenderLayers,
    reflect::TypePath,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

pub struct FogPlugin;

impl Plugin for FogPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FogMaterial>::default());
        app.add_systems(Startup, (
            spawn_fog,
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct FogMaterial {}

impl Material2d for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

fn spawn_fog(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<FogMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(500.0, 100.0))),
        MeshMaterial2d(materials.add(FogMaterial {})),
        Transform::from_xyz(0.0, 10.0, 0.0),
        RenderLayers::layer(1),
    ));
}