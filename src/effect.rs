use bevy::prelude::*;

use crate::mixer::AudioPlusMixerChannel;

#[derive(Clone)]
pub struct AudioPlusSoundEffect {
    pub(crate) audio_sources: Vec<Handle<bevy_kira_audio::AudioSource>>,
    pub(crate) voices: usize,
    pub(crate) positional: bool,
    pub(crate) volume: f32,
    pub(crate) volume_variation: f32,
    pub(crate) pitch: f32,
    pub(crate) pitch_variation: f32,
    pub(crate) distance: f32,
    pub(crate) chance: f32,
    pub(crate) fade_in: f32,
    pub(crate) fade_out: f32,
    pub(crate) channel: AudioPlusMixerChannel,
}

impl Default for AudioPlusSoundEffect {
    fn default() -> Self {
        Self {
            audio_sources: vec![],
            voices: 1,
            positional: false,
            volume: 1.,
            volume_variation: 0.,
            pitch: 1.,
            pitch_variation: 0.,
            distance: 1000.,
            chance: 1.,
            fade_in: 0.,
            fade_out: 0.,
            channel: AudioPlusMixerChannel::None,
        }
    }
}

impl AudioPlusSoundEffect {
    pub fn single(audio_source: Handle<bevy_kira_audio::AudioSource>) -> Self {
        Self::multiple(vec![audio_source])
    }

    pub fn multiple(audio_sources: Vec<Handle<bevy_kira_audio::AudioSource>>) -> Self {
        Self {
            audio_sources,
            ..Default::default()
        }
    }

    pub fn with_voices(self, voices: usize) -> Self {
        Self { voices, ..self }
    }

    pub fn with_positional(self, positional: bool) -> Self {
        Self { positional, ..self }
    }

    pub fn with_volume(self, volume: f32, volume_variation: f32) -> Self {
        Self {
            volume,
            volume_variation,
            ..self
        }
    }

    pub fn with_pitch(self, pitch: f32, pitch_variation: f32) -> Self {
        Self {
            pitch,
            pitch_variation,
            ..self
        }
    }

    pub fn with_distance(self, distance: f32) -> Self {
        Self { distance, ..self }
    }

    pub fn with_chance(self, chance: f32) -> Self {
        Self { chance, ..self }
    }

    pub fn with_fade(self, fade_in: f32, fade_out: f32) -> Self {
        Self {
            fade_in,
            fade_out,
            ..self
        }
    }

    pub fn with_channel(self, channel: AudioPlusMixerChannel) -> Self {
        Self { channel, ..self }
    }
}

impl From<Handle<bevy_kira_audio::AudioSource>> for AudioPlusSoundEffect {
    fn from(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        AudioPlusSoundEffect::single(resource)
    }
}
