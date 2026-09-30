mod effects;
mod gameplay;
mod hud;

use crate::effects::EffectsPlugin;
use crate::gameplay::GameplayPlugin;
use crate::hud::HUDPlugin;

use bevy::prelude::*;

// TODO add screen shake
// TODO add gamma to posterization
// TODO add random particles
// TODO store collision vector so you cant jump into what you're jumping off of
// TODO eating mushrooms
// TODO 3D color split shader

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EffectsPlugin,
            GameplayPlugin,
            HUDPlugin,
        ))
        .run();
}