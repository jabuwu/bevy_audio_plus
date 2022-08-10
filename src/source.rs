use crate::{
    effect::AudioPlusSoundEffect,
    prelude::AudioPlusListener,
    voice::{AudioPlusVoice, AudioPlusVoiceState},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct AudioPlusSource {
    pub(crate) sound_effect: AudioPlusSoundEffect,
    pub(crate) voices: Vec<AudioPlusVoice>,
    pub(crate) next_voice: usize,
}

impl AudioPlusSource {
    pub fn new(sound_effect: AudioPlusSoundEffect) -> Self {
        let mut voices = vec![];
        for _ in 0..sound_effect.voices {
            voices.push(AudioPlusVoice::new());
        }
        Self {
            sound_effect,
            voices,
            next_voice: 0,
        }
    }

    pub fn as_playing(mut self) -> Self {
        self.play();
        self
    }

    pub fn as_looping(mut self) -> Self {
        self.play_looped();
        self
    }

    fn prepare_voice(&mut self) -> Option<usize> {
        if !self.voices.is_empty() && !self.sound_effect.audio_sources.is_empty() {
            let id = self.next_voice;
            self.next_voice = (self.next_voice + 1) % self.voices.len();
            let voice = &mut self.voices[id];
            let audio_source = &self.sound_effect.audio_sources
                [rand::random::<usize>() % self.sound_effect.audio_sources.len()];
            voice.reset();
            voice.audio_source = Some(audio_source.clone());
            voice.state_dirty = true;
            voice.volume = (self.sound_effect.volume - self.sound_effect.volume_variation * 0.5
                + rand::random::<f32>() * self.sound_effect.volume_variation)
                .clamp(0., 1.);
            voice.playback_rate = (self.sound_effect.pitch
                - self.sound_effect.pitch_variation * 0.5
                + rand::random::<f32>() * self.sound_effect.pitch_variation)
                .max(0.);
            Some(id)
        } else {
            None
        }
    }

    pub fn play(&mut self) {
        if let Some(index) = self.prepare_voice() {
            self.voices[index].state = AudioPlusVoiceState::Playing;
        }
    }

    pub fn play_looped(&mut self) {
        if let Some(index) = self.prepare_voice() {
            self.voices[index].state = AudioPlusVoiceState::Looping;
        }
    }

    pub fn stop(&mut self) {
        for voice in self.voices.iter_mut() {
            if voice.state != AudioPlusVoiceState::Stopped {
                voice.reset();
                voice.state_dirty = true;
            }
        }
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
    let dist = 1000.;
    for (mut source, transform) in queries.p0().iter_mut() {
        let mut volume = 1.;
        let mut panning = 0.5;
        if source.sound_effect.positional && transform.is_some() && listener_transform.is_some() {
            let relative_position = transform.unwrap().translation.truncate()
                - listener_transform.unwrap().translation.truncate();
            let distance = relative_position.length();
            volume *= ((dist - distance) / dist).clamp(0., 1.);
            panning = (0.5 + relative_position.x / dist).clamp(0.2, 0.8);
        }
        for voice in source.voices.iter_mut() {
            if voice.status.initialized && !voice.status.playing {
                voice.reset();
                voice.state_dirty = true;
            } else {
                voice.should_assign = voice.state != AudioPlusVoiceState::Stopped;
                voice.volume_multiplier = volume;
                voice.panning = panning;
            }
        }
    }
}
