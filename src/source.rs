use crate::{
    channels::{ChannelSettings, ChannelState},
    prelude::AudioPlusListener,
    sound::AudioPlusSoundGroup,
};
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AudioPlusSource {
    pub(crate) sound_group: AudioPlusSoundGroup,
    pub(crate) randomize: bool,
    pub(crate) positional: bool,
    pub(crate) volume: f32,
    pub(crate) state: ChannelState,
    pub(crate) state_counter: u32,
    pub(crate) channel_settings: ChannelSettings,
}

impl AudioPlusSource {
    pub fn new(sound_group: AudioPlusSoundGroup) -> Self {
        Self {
            sound_group,
            randomize: false,
            positional: false,
            volume: 1.,
            state: ChannelState::Stopped,
            state_counter: 1,
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
            randomize: true,
            ..self
        }
    }

    pub fn with_looping(self) -> Self {
        Self {
            state: ChannelState::Looping,
            randomize: true,
            ..self
        }
    }

    pub fn play(&mut self) {
        self.randomize = true;
        self.state = ChannelState::Playing;
        self.state_counter += 1;
    }

    pub fn play_looped(&mut self) {
        self.randomize = true;
        self.state = ChannelState::Looping;
        self.state_counter += 1;
    }

    pub fn stop(&mut self) {
        self.randomize = true;
        self.state = ChannelState::Stopped;
        self.state_counter += 1;
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
        if source.randomize {
            if source.sound_group.sounds.len() > 0 {
                let sound = source.sound_group.sounds
                    [rand::random::<usize>() % source.sound_group.sounds.len()]
                .clone();
                source.channel_settings.audio_source = Some(sound.resource.clone());
                source.channel_settings.playback_rate = 1.0 - sound.pitch_variation * 0.5
                    + rand::random::<f32>() * sound.pitch_variation;
            }
            source.randomize = false;
        }
        let dist = 1000.;
        source.channel_settings.should_assign = true;
        source.channel_settings.volume = source.volume;
        source.channel_settings.state = source.state;
        source.channel_settings.state_counter = source.state_counter;
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
