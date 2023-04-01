use audio_plus::prelude::*;
use bevy::{prelude::*, window::WindowResolution};
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - Mixer".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AudioPlusPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(InstructionsPlugin(
            "WASD to move\nPress F to toggle all SFX\nPress M to toggle all music".to_owned(),
        ))
        .add_startup_system(init)
        .add_system(controls)
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
                .with_distance(350.)
                .with_channel(AudioPlusMixerChannel::Music),
        )
        .as_looping(),
    ));
    commands.spawn((
        box_sprite(Vec2::new(-300., 0.), Color::BLUE),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/music_2.ogg"))
                .with_positional(true)
                .with_distance(350.)
                .with_channel(AudioPlusMixerChannel::Music),
        )
        .as_looping(),
    ));
    commands.spawn((
        box_sprite(Vec2::new(0., -200.), Color::ORANGE),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/rock.ogg"))
                .with_positional(true)
                .with_distance(150.)
                .with_channel(AudioPlusMixerChannel::Sfx),
        )
        .as_looping(),
    ));
    commands.spawn((
        box_sprite(Vec2::new(200., -100.), Color::ORANGE),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/scissors.ogg"))
                .with_positional(true)
                .with_distance(150.)
                .with_channel(AudioPlusMixerChannel::Sfx),
        )
        .as_looping(),
    ));
    commands.spawn((
        box_sprite(Vec2::new(-100., 100.), Color::ORANGE),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/paper.ogg"))
                .with_positional(true)
                .with_distance(150.)
                .with_channel(AudioPlusMixerChannel::Sfx),
        )
        .as_looping(),
    ));
}

fn controls(input: Res<Input<KeyCode>>, mut mixer: ResMut<AudioPlusMixer>) {
    if input.just_pressed(KeyCode::F) {
        if mixer.get_volume(AudioPlusMixerChannel::Sfx) == 0. {
            mixer.set_volume(AudioPlusMixerChannel::Sfx, 1.);
        } else {
            mixer.set_volume(AudioPlusMixerChannel::Sfx, 0.);
        }
    }
    if input.just_pressed(KeyCode::M) {
        if mixer.get_volume(AudioPlusMixerChannel::Music) == 0. {
            mixer.set_volume(AudioPlusMixerChannel::Music, 1.);
        } else {
            mixer.set_volume(AudioPlusMixerChannel::Music, 0.);
        }
    }
}
