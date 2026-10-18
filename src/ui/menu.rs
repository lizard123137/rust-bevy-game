use bevy::prelude::*;

use crate::{
    GameState,
    ui::components,
};

pub struct UIMenuPlugin;

impl Plugin for UIMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.add_systems(OnEnter(GameState::Menu), menu_setup);
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    #[default]
    Main,
    Disabled,
}


fn menu_setup (
    mut commands: Commands,
) {
    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Camera2d,
    ));

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
        },
        children![(
            Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
            },
            children![
                (
                    Text::new("Frog Game"),
                    TextFont {
                        font_size: FontSize::Px(67.0),
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                (
                    Button,
                    components::ButtonAction::MenuGame,
                    children![(
                        Text::new("Game"),
                    )],
                ),
                (
                    Button,
                    components::ButtonAction::MenuEditor,
                    children![(
                        Text::new("Editor"),
                    )],
                ),
                (
                    Button,
                    components::ButtonAction::MenuQuit,
                    children![(
                        Text::new("Quit"),
                    )]
                ),
            ],
        )]
    ));
}