mod effects;
mod gameplay;
mod hud;

use crate::effects::EffectsPlugin;
use crate::gameplay::GameplayPlugin;
use crate::hud::HUDPlugin;

use bevy::prelude::*;

// For FX add vignette, screen shake and pixelated bloom

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