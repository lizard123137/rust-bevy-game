mod asset_loader;
mod editor;
mod gameplay;
mod graphics;
mod level;
mod ui;

use crate::asset_loader::AssetLoaderPlugin;
use crate::editor::EditorPlugin;
use crate::gameplay::GameplayPlugin;
use crate::graphics::GraphicsPlugin;
use crate::ui::hud::HUDPlugin;
use crate::ui::menu::MenuPlugin;

use bevy::prelude::*;

// TODO gameplay
// TODO     Its a horror game, add enemies and sound effects
// TODO     Only grapple to rigidbodies with movable==false
// TODO     Store collision vector so you cant jump into what you're jumping off of
// TODO     Controller support (maybe autoaim)
// TODO     Talking with druid NPC
// TODO     Teleportation by digging into soft ground
// TODO     Add sprite sheet animation
// TODO     Eating mushrooms gives you power ups but makes the frog more paranoid
// TODO     Paranoia intensifies the horror elements (Maybe add eye sprites that open as it increases)
// TODO     Sign dialogue type

// TODO visual fx
// TODO     Add screen shake
// TODO     Add random particles
// TODO     3D color split shader
// TODO     Make water a fullscreen shader and only control surface height
// TODO     Rain
// TODO     Parallax background
// TODO     Optimize spritesheet animations to cache them as a resource
// TODO     Make fog shader fade with material edges

// TODO developer tools
// TODO     Make the game hot reloadable to change configs during development

// TODO tree generator
// TODO     Make trees generate randomly from files that describe their structure
// TODO     Like shrubbery by tantan, but in 2D

// TODO world editor
// TODO     Selectable menu with different types of tiles you can place
// TODO     Make it snap to the grid of the world
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
            AssetLoaderPlugin,
            MenuPlugin,
            EditorPlugin,
            GameplayPlugin,
            GraphicsPlugin,
            HUDPlugin,
        ))
        .run();
}