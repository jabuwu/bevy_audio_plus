use bevy::{prelude::*, window::WindowResolution};
use bevy_audio_plus::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - Random".to_string(),
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
            InstructionsPlugin("Press SPACE to play a random sound".to_owned()),
        ))
        .add_systems(Startup, init)
        .add_systems(Update, controls)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds = AudioPlusSoundEffect::multiple(vec![
        asset_server.load("sounds/rock.ogg"),
        asset_server.load("sounds/paper.ogg"),
        asset_server.load("sounds/scissors.ogg"),
    ])
    .with_voices(3)
    .with_pitch(1., 0.2)
    .with_volume(0.5, 0.5);
    commands.spawn(Camera2dBundle::default());
    commands.spawn(AudioPlusSource::new(sounds));
}

fn controls(input: Res<Input<KeyCode>>, mut query: Query<&mut AudioPlusSource>) {
    if input.just_pressed(KeyCode::Space) {
        for mut source in query.iter_mut() {
            source.play();
        }
    }
}
