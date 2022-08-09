use bevy::prelude::*;

#[derive(Clone)]
pub struct AudioPlusSoundEffect {
    pub(crate) audio_sources: Vec<Handle<bevy_kira_audio::AudioSource>>,
    pub(crate) voices: usize,
    pub(crate) positional: bool,
    pub(crate) volume: f32,
    pub(crate) volume_variation: f32,
    pub(crate) pitch: f32,
    pub(crate) pitch_variation: f32,
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
}

impl From<Handle<bevy_kira_audio::AudioSource>> for AudioPlusSoundEffect {
    fn from(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        AudioPlusSoundEffect::single(resource)
    }
}
