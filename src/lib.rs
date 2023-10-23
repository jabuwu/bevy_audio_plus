use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use channels::add_audio_channels;
use mixer::AudioPlusMixer;

pub use bevy_kira_audio;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub enum AudioPlusSystem {
    UpdateAudioSources,
    Debug,
}

pub struct AudioPlusPlugin;

impl Plugin for AudioPlusPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .init_resource::<AudioPlusMixer>()
            .add_systems(
                Update,
                source::update_audio_sources.in_set(AudioPlusSystem::UpdateAudioSources),
            );
        add_audio_channels(app);
    }
}

pub mod channels;
pub mod effect;
pub mod listener;
pub mod mixer;
pub mod prelude;
pub mod source;
pub mod voice;
