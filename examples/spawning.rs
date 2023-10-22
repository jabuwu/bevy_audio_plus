use bevy::{prelude::*, window::WindowResolution};
use bevy_audio_plus::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - Spawning".to_string(),
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
            InstructionsPlugin("WASD to move".to_owned()),
        ))
        .add_systems(Startup, init)
        .add_systems(Update, spawn)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        box_sprite(Vec2::ZERO, Color::GREEN),
        Player,
        AudioPlusListener,
    ));
}

#[derive(Default)]
struct SpawnData {
    spawn_time: f32,
}

fn spawn(
    mut commands: Commands,
    mut data: Local<SpawnData>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    let time_to_spawn = 1.;
    data.spawn_time += time.delta_seconds();
    if data.spawn_time >= time_to_spawn {
        let x = rand::random::<f32>() * 1000. - 500.;
        let y = rand::random::<f32>() * 600. - 300.;

        commands.spawn((
            box_sprite(Vec2::new(x, y), Color::BLUE),
            AudioPlusSource::new(
                AudioPlusSoundEffect::single(asset_server.load("sounds/pong.ogg"))
                    .with_positional(true)
                    .with_pitch(1., 0.2),
            )
            .as_playing(),
            TimeToLive(0.8),
        ));
        data.spawn_time = 0.;
    }
}
