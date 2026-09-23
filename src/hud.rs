use bevy::prelude::*;

pub struct HUDPlugin;

impl Plugin for HUDPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_text,
            spawn_vignette,
        ));
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

fn spawn_vignette(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            left: Val::Px(0.0),
            top: Val::Px(0.0),
            ..default()
        },
        // Pure black radial gradient fading out from center
        BackgroundGradient::from(RadialGradient {
            color_space: InterpolationColorSpace::Srgba,
            stops: vec![
                ColorStop::new(Srgba::NONE, percent(0)),
                ColorStop::new(Srgba::NONE, percent(50)),
                ColorStop::new(Srgba::new(0.0, 0.0, 0.0, 0.8), percent(100)),
            ],
            ..default()
        }),
        // Ensure UI overlay passes click input through to the game world
        Interaction::None,
    ));
}