pub mod fog;
pub mod lofi;
pub mod water;
pub mod vignette;

use bevy::{
    asset::uuid_handle,
    asset::load_internal_asset,
    prelude::*,
    shader::Shader,
};

const NOISE_SHADER_HANDLE: Handle<Shader> = uuid_handle!("c3b6a1e0-4f2a-4b8e-9c3d-1a2b3c4d5e6f"); // TODO store IDs in enum

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(app, NOISE_SHADER_HANDLE, "../../assets/shaders/noise.wgsl", Shader::from_wgsl);

        app.add_plugins((
            fog::FogPlugin,
            lofi::LofiPlugin,
            water::WaterPlugin,
            vignette::VignettePlugin,
        ));
    }
}