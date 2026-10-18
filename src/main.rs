mod asset_loader;
mod audio;
mod editor;
mod gameplay;
mod graphics;
mod level;
mod ui;

use crate::asset_loader::AssetLoaderPlugin;
use crate::audio::AudioPlugin;
use crate::editor::EditorPlugin;
use crate::gameplay::GameplayPlugin;
use crate::graphics::GraphicsPlugin;
use crate::ui::UIPlugin;

use bevy::prelude::*;

// TODO story
// TODO     Maybe make it a climbing a tree / descending into a hole game
// TODO     Its a horror game, add enemies
// TODO     Talking with druid NPC
// TODO     Sign dialogue type

// TODO gameplay
// TODO     Store collision vector so you cant jump into what you're jumping off of
// TODO     Controller support (maybe autoaim)
// TODO     Teleportation by digging into soft ground
// TODO     Eating mushrooms gives you power ups but makes the frog more paranoid
// TODO     Paranoia intensifies the horror elements (Maybe add eye sprites that open as it increases)
// TODO     Snap objects to 16px grid before spawning (right now they are offset by 8px)

// TODO audio
// TODO     Make a system where sfx can be assigned to events
// TODO     Map SFX to events through RON
// TODO     Add atmospheric sounds when frog is paranoid
// TODO     Maybe switch to .ogg from .wav

// TODO visual fx
// TODO     Add screen shake
// TODO     Add random particles
// TODO     3D color split shader
// TODO     Make water a fullscreen shader and only control surface height
// TODO     Rain shader
// TODO     Parallax background
// TODO     Optimize spritesheet animations to cache them as a resource
// TODO     Make fog shader fade with material edges
// TODO     Add sprite sheet animation

// TODO developer tools / code improvements
// TODO     Make the game hot reloadable to change configs during development
// TODO     Refactor physics engine into multiple files
// TODO     Somehow order systems to run in a way that makes sense

// TODO tree generator (maybe... its a bit over the top)
// TODO     Make trees generate randomly from files that describe their structure
// TODO     Like shrubbery by tantan, but in 2D

// TODO world editor
// TODO     Selectable menu with different types of tiles you can place

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
            AudioPlugin,
            EditorPlugin,
            GameplayPlugin,
            GraphicsPlugin,
            UIPlugin,
        ))
        .run();
}