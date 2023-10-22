use bevy::{prelude::*, window::WindowResolution};
use bevy_audio_plus::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - Positional".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            AudioPlusPlugin,
            PlayerPlugin,
            InstructionsPlugin("WASD to move".to_owned()),
        ))
        .add_systems(Startup, init)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        box_sprite(Vec2::ZERO, Color::GREEN),
        Player,
        AudioPlusListener,
    ));
    commands.spawn((
        box_sprite(Vec2::new(300., 0.), Color::BLUE),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/music_1.ogg"))
                .with_positional(true)
                .with_distance(250.),
        )
        .as_looping(),
    ));
    commands.spawn((
        box_sprite(Vec2::new(-300., 0.), Color::BLUE),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/music_2.ogg"))
                .with_positional(true)
                .with_distance(250.),
        )
        .as_looping(),
    ));
}
