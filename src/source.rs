use crate::{
    channels::{ChannelSettings, ChannelState},
    prelude::AudioPlusListener,
};
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AudioPlusSource {
    pub(crate) resource: Handle<bevy_kira_audio::AudioSource>,
    pub(crate) positional: bool,
    pub(crate) volume: f32,
    pub(crate) state: ChannelState,
    pub(crate) channel_settings: ChannelSettings,
}

impl AudioPlusSource {
    pub fn new(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        Self {
            resource,
            positional: false,
            volume: 1.,
            state: ChannelState::Stopped,
            channel_settings: ChannelSettings::default(),
        }
    }

    pub fn with_positional(self) -> Self {
        Self {
            positional: true,
            ..self
        }
    }

    pub fn with_playing(self) -> Self {
        Self {
            state: ChannelState::Playing,
            ..self
        }
    }

    pub fn with_looping(self) -> Self {
        Self {
            state: ChannelState::Looping,
            ..self
        }
    }

    pub fn play(&mut self) {
        self.state = ChannelState::Playing;
    }

    pub fn play_looped(&mut self) {
        self.state = ChannelState::Looping;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume;
    }
}

pub(crate) fn update_audio_sources(
    mut queries: ParamSet<(
        Query<(&mut AudioPlusSource, Option<&Transform>)>,
        Query<&Transform, With<AudioPlusListener>>,
    )>,
) {
    let listener_transform = if let Ok(transform) = queries.p1().get_single() {
        Some(*transform)
    } else {
        None
    };
    for (mut source, transform) in queries.p0().iter_mut() {
        let dist = 1000.;
        source.channel_settings.should_assign = true;
        source.channel_settings.volume = source.volume;
        source.channel_settings.state = source.state;
        if source.positional && transform.is_some() && listener_transform.is_some() {
            let relative_position = transform.unwrap().translation.truncate()
                - listener_transform.unwrap().translation.truncate();
            let distance = relative_position.length();
            let distance_volume = ((dist - distance) / dist).clamp(0., 1.);
            let distance_panning = (0.5 + relative_position.x / dist).clamp(0.2, 0.8);
            source.channel_settings.volume *= distance_volume;
            source.channel_settings.panning = distance_panning;
        } else {
            source.channel_settings.panning = 0.5;
        }
    }
}
