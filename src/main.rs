mod effects;
mod gameplay;
mod hud;

use crate::effects::EffectsPlugin;
use crate::gameplay::GameplayPlugin;
use crate::hud::HUDPlugin;

use bevy::prelude::*;

// TODO add screen shake
// TODO add random particles
// TODO store collision vector so you cant jump into what you're jumping off of
// TODO eating mushrooms
// TODO 3D color split shader

// TODO tree generator
// TODO      Make trees generate randomly from files that describe their structure
// TODO      Like shrubbery by tantan, but in 2D

// TODO world editor
// TODO      Selectable menu with different types of tiles you can place
// TODO      Make it snap to the grid of the world
// TODO      Save it to some kind of file that can be opened and edited

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EffectsPlugin,
            GameplayPlugin,
            HUDPlugin,
        ))
        .run();
}