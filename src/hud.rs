use bevy::prelude::*;

use crate::GameState;
use crate::gameplay::player::Frog;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (
            spawn_text,
        ));
        app.add_systems(Update, (
            update_text.run_if(in_state(GameState::Game)),
        ));
    }
}

#[derive(Debug, Component)]
struct DebugText;

fn spawn_text(mut commands: Commands) {
    let font = TextFont {
        font_size: FontSize::Px(25.0),
        ..default()
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::new("0.00 0.00"),
                font,
                DebugText,
            ),
        ],
    ));
}

fn update_text(
    mut text: Single<&mut Text, With<DebugText>>,
    frog: Single<&Frog>,
) {
    text.0 = format!("{:.2} {:.2}", frog.target_pos.x, frog.target_pos.y);
}