use bevy::prelude::*;
use crate::GameState;

pub struct VignettePlugin;

impl Plugin for VignettePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), (
            spawn_vignette,
        ));
    }
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