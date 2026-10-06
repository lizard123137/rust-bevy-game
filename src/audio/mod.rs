use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(
            |sound: On<PlaySound>, asset_server: Res<AssetServer>, mut commands: Commands| {
                let sfx = match sound.name.as_str() {
                    "jump" => "sounds/jump.wav",
                    "grapple" => "sounds/lick.wav",
                    _ => "",
                };

                commands.spawn((
                    AudioPlayer::new(asset_server.load(sfx)),
                    PlaybackSettings::DESPAWN,
                ));
            }
        );
    }
}

#[derive(Event)]
pub struct PlaySound {
    pub name: String,
}