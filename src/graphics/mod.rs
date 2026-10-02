pub mod animation;
pub mod effects;

use animation::AnimationPlugin;
use effects::EffectsPlugin;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            AnimationPlugin,
            EffectsPlugin,
        ));
    }
}