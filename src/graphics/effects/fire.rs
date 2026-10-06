use bevy::{
    reflect::TypePath,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

pub struct FirePlugin;

impl Plugin for FirePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FireMaterial>::default());
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FireMaterial {}

impl Material2d for FireMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fire.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}