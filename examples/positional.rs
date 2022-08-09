use audio_plus::prelude::*;
use bevy::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - Positional".to_string(),
            width: 1280.,
            height: 720.,
            resizable: false,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlusPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(TimeToLivePlugin)
        .add_plugin(InstructionsPlugin("WASD to move".to_owned()))
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands
        .spawn_bundle(box_sprite(Vec2::new(450., 250.), Color::GREEN))
        .insert(Player)
        .insert(AudioPlusListener);
    commands
        .spawn_bundle(box_sprite(Vec2::new(500., 300.), Color::BLUE))
        .insert(
            AudioPlusSource::new(asset_server.load("sounds/music_1.ogg").into())
                .with_positional()
                .with_looping(),
        );
    commands
        .spawn_bundle(box_sprite(Vec2::new(-500., -300.), Color::BLUE))
        .insert(
            AudioPlusSource::new(asset_server.load("sounds/music_2.ogg").into())
                .with_positional()
                .with_looping(),
        )
        .insert(TimeToLive(30.));
}
