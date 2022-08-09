use audio_plus::prelude::*;
use bevy::prelude::*;
use examples_common::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Audio Plus - Random".to_string(),
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
        .add_plugin(InstructionsPlugin(
            "Press SPACE to play a random sound".to_owned(),
        ))
        .add_startup_system(init)
        .add_system(sound_play)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sounds = AudioPlusSoundGroup::new(vec![
        AudioPlusSound::new(asset_server.load("sounds/rock.ogg")).with_pitch_variation(0.5),
        AudioPlusSound::new(asset_server.load("sounds/paper.ogg")).with_pitch_variation(0.5),
        AudioPlusSound::new(asset_server.load("sounds/scissors.ogg")).with_pitch_variation(0.5),
    ]);
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn().insert(AudioPlusSource::new(sounds));
}

fn sound_play(input: Res<Input<KeyCode>>, mut query: Query<&mut AudioPlusSource>) {
    if input.just_pressed(KeyCode::Space) {
        for mut source in query.iter_mut() {
            source.play();
        }
    }
}
