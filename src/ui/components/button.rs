use bevy::{
    color::palettes::basic::*,
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
};

use crate::{
    GameState,
    ui::menu::MenuState,
    editor::events::{
        EditorLayerSelect,
        EditorSave,
    },
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// TODO customize this... For now its ripped straight from the official bevy examples 

pub fn spawn_button(
    text: &str,
    size: IVec2,
) -> impl Bundle {
    (
        Button,
        Hovered::default(),
        Node {
            width: px(size.x),
            height: px(size.y),
            border: UiRect::all(px(5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

pub fn update_buttons_visual(
    mut buttons: Query<
        (
            &Hovered,
            Has<Pressed>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<Button>,
    >,
) {
    for (hovered, pressed, mut color, mut border_color, children) in &mut buttons {
        match (hovered.get(), pressed) {
            // Pressed (and, since you can only press what you're hovering, also hovered).
            (_, true) => {
                *color = PRESSED_BUTTON.into();
                border_color.set_all(RED);
            }
            // Hovered but not pressed.
            (true, false) => {
                *color = HOVERED_BUTTON.into();
                border_color.set_all(WHITE);
            }
            // Neither hovered nor pressed.
            (false, false) => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(BLACK);
            }
        }
    }
}

pub fn update_buttons_action(
    interaction_query: Query<
        (&Interaction, &ButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for (interaction, button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action {
                ButtonAction::MenuQuit => {
                    app_exit_writer.write(AppExit::Success);
                }
                ButtonAction::MenuGame => {
                    game_state.set(GameState::Game);
                    menu_state.set(MenuState::Disabled);
                }
                ButtonAction::MenuEditor => {
                    game_state.set(GameState::Editor);
                    menu_state.set(MenuState::Disabled);
                }

                ButtonAction::EditorSave => {
                    commands.trigger(EditorSave);
                },
                ButtonAction::EditorLayerSelect(layer) => {
                    commands.trigger(EditorLayerSelect { layer: *layer });
                }
            }
        }
    }
}

#[derive(Component)]
pub enum ButtonAction {
    MenuGame,
    MenuEditor,
    MenuQuit,

    EditorSave,
    EditorLayerSelect(f32),
}