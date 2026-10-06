use bevy::{
    reflect::TypePath,
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
};

pub struct MaterialShadersPlugin;

impl Plugin for MaterialShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<FireMaterial>::default());
        app.add_plugins(Material2dPlugin::<FogMaterial>::default());
    }
}

//
// Fire
//

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

//
// Fog
//

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct FogMaterial {}

impl Material2d for FogMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fog.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}