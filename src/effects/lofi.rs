use bevy::{
    core_pipeline::fullscreen_material::{FullscreenMaterial, FullscreenMaterialPlugin},
    prelude::*,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
    shader::ShaderRef,
};

pub struct LofiPlugin;

impl Plugin for LofiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FullscreenMaterialPlugin::<LofiEffect>::default());
    }
}

#[derive(Component, ExtractComponent, Clone, Copy, ShaderType, Default)]
pub struct LofiEffect {
    pixel_size: f32,
    color_levels: f32,
    gamma: Vec2,
}

impl LofiEffect {
    pub fn new(pixel_size: f32, color_levels: f32, gamma: Vec2) -> Self {
        Self {
            pixel_size: pixel_size,
            color_levels: color_levels,
            gamma: gamma,
            ..Default::default()
        }
    }
}

impl FullscreenMaterial for LofiEffect {
    fn fragment_shader() -> ShaderRef {
        "shaders/lofi.wgsl".into()
    }

    fn schedule() -> impl bevy::ecs::schedule::ScheduleLabel + Clone {
        bevy::core_pipeline::Core2d
    }

    fn schedule_configs(
        system: bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem>,
    ) -> bevy::ecs::schedule::ScheduleConfigs<bevy::ecs::system::BoxedSystem> {
        system
            .in_set(bevy::core_pipeline::Core2dSystems::PostProcess)
            .before(bevy::core_pipeline::tonemapping::tonemapping)
    }
}