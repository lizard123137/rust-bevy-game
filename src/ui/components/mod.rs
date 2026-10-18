pub mod button;

pub use button::{
    ButtonAction,
    spawn_button
};

use button::{
    update_buttons_action,
    update_buttons_visual,
};

use bevy::prelude::*;

pub struct UIComponentsPlugin;

impl Plugin for UIComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_buttons_action,
            update_buttons_visual,
        ));
    }
}