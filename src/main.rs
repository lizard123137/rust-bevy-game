mod editor;
mod effects;
mod gameplay;
mod ui;
mod hud;

use crate::editor::EditorPlugin;
use crate::effects::EffectsPlugin;
use crate::gameplay::GameplayPlugin;
use crate::hud::HUDPlugin;
use crate::ui::menu::MenuPlugin;

use bevy::prelude::*;

// TODO gameplay
// TODO     only grapple to rigidbodies with movable==false
// TODO     store collision vector so you cant jump into what you're jumping off of
// TODO     controller support (maybe autoaim)
// TODO     eating mushrooms
// TODO     talking with druid NPC

// TODO visual fx
// TODO     add screen shake
// TODO     add random particles
// TODO     3D color split shader
// TODO     make water a fullscreen shader and only control surface height

// TODO developer tools
// TODO     Make the game hot reloadable to change configs during development

// TODO tree generator
// TODO     Make trees generate randomly from files that describe their structure
// TODO     Like shrubbery by tantan, but in 2D

// TODO world editor
// TODO     Selectable menu with different types of tiles you can place
// TODO     Make it snap to the grid of the world
// TODO     Save it to some kind of file that can be opened and edited
// TODO     Somehow bunch colliders of tiles together

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
    Editor,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_plugins((
            MenuPlugin,
            EditorPlugin,
            EffectsPlugin,
            GameplayPlugin,
            HUDPlugin,
        ))
        .run();
}