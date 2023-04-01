use audio_plus::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
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
        .add_plugin(AudioPlusPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TimeToLivePlugin)
        .add_plugin(InstructionsPlugin(
            "Press SPACE to play a random sound".to_owned(),
        ))
        .add_startup_system(init)
        .add_system(controls)
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
