use bevy::{prelude::*, window::WindowResolution};
use bevy_audio_plus::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - Fade".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            AudioPlusPlugin,
            PlayerPlugin,
            TimeToLivePlugin,
            InstructionsPlugin("Press SPACE to play sound\nPress S to stop sound".to_owned()),
        ))
        .add_systems(Startup, init)
        .add_systems(Update, controls)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(AudioPlusSource::new(
        AudioPlusSoundEffect::single(asset_server.load("sounds/music_1.ogg")).with_fade(3., 3.),
    ));
}

fn controls(input: Res<Input<KeyCode>>, mut query: Query<&mut AudioPlusSource>) {
    if input.just_pressed(KeyCode::Space) {
        for mut source in query.iter_mut() {
            source.play_looped();
        }
    }
    if input.just_pressed(KeyCode::S) {
        for mut source in query.iter_mut() {
            source.stop();
        }
    }
}
