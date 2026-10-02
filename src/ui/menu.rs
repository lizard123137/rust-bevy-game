use bevy::prelude::*;
use crate::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.add_systems(OnEnter(GameState::Menu), menu_setup);
        app.add_systems(Update,
            menu_action.run_if(in_state(GameState::Menu))
        );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    #[default]
    Main,
    Disabled,
}

#[derive(Component)]
enum MenuButtonAction {
    Game,
    Editor,
    Quit,
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
                    MenuButtonAction::Game,
                    children![(
                        Text::new("Game"),
                    )],
                ),
                (
                    Button,
                    MenuButtonAction::Editor,
                    children![(
                        Text::new("Editor"),
                    )],
                ),
                (
                    Button,
                    MenuButtonAction::Quit,
                    children![(
                        Text::new("Quit"),
                    )]
                ),
            ],
        )]
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_writer.write(AppExit::Success);
                }
                MenuButtonAction::Game => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Editor => {
                    game_state.set(GameState::Editor);
                    menu_state.set(MenuState::Disabled);
                }
            }
        }
    }
}