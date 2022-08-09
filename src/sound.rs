use bevy::prelude::*;

#[derive(Clone)]
pub struct AudioPlusSound {
    pub(crate) resource: Handle<bevy_kira_audio::AudioSource>,
    pub(crate) pitch_variation: f32,
}

impl AudioPlusSound {
    pub fn new(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        Self {
            resource,
            pitch_variation: 0.,
        }
    }

    pub fn with_pitch_variation(self, pitch_variation: f32) -> Self {
        Self {
            pitch_variation,
            ..self
        }
    }
}

impl From<Handle<bevy_kira_audio::AudioSource>> for AudioPlusSound {
    fn from(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        AudioPlusSound::new(resource)
    }
}

#[derive(Clone)]
pub struct AudioPlusSoundGroup {
    pub(crate) sounds: Vec<AudioPlusSound>,
}

impl AudioPlusSoundGroup {
    pub fn new(sounds: Vec<AudioPlusSound>) -> Self {
        Self { sounds }
    }
}

impl From<Handle<bevy_kira_audio::AudioSource>> for AudioPlusSoundGroup {
    fn from(resource: Handle<bevy_kira_audio::AudioSource>) -> Self {
        AudioPlusSoundGroup::new(vec![AudioPlusSound::new(resource)])
    }
}

impl From<AudioPlusSound> for AudioPlusSoundGroup {
    fn from(sound: AudioPlusSound) -> Self {
        AudioPlusSoundGroup::new(vec![sound])
    }
}
