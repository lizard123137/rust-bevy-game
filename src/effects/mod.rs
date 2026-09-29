pub mod water;
pub mod vignette;

use bevy::prelude::*;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            water::WaterPlugin,
            vignette::VignettePlugin,
        ));
    }
}