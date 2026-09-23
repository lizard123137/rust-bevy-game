use bevy::prelude::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_text);
    }
}

#[derive(Component)]
struct ThrottleText;

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
                Text::new("Hello World"),
                font,
                ThrottleText,
            ),
        ],
    ));
}