use audio_plus::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - BGM".to_string(),
                resolution: WindowResolution::new(1280., 720.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(AudioPlusPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(AudioPlusSource::new(asset_server.load("sounds/music_1.ogg").into()).as_looping());
}
