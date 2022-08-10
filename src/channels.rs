use crate::{
    source::AudioPlusSource,
    voice::{AudioPlusVoiceHandle, AudioPlusVoiceState},
    AudioPlusSystems,
};
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, InstanceHandle};

macro_rules! channels {
    ( $( $x:ident ),* ) => {
        $(
            pub struct $x;
        )*
        pub fn add_audio_channels(app: &mut App) {
            $(
                app.add_audio_channel::<$x>();
                app.add_system(update_kira_channel::<$x>.after(AudioPlusSystems::UpdateAudioSources));
            )*
        }
    };
}

#[derive(Default)]
struct ChannelData {
    initialized: bool,
    voice_handle: Option<AudioPlusVoiceHandle>,
    instance_handle: Option<InstanceHandle>,
}

fn update_kira_channel<T: Resource>(
    mut data: Local<ChannelData>,
    channel: Res<AudioChannel<T>>,
    mut query: Query<(Entity, &mut AudioPlusSource)>,
) {
    if !data.initialized {
        channel.set_volume(0.);
        data.initialized = true;
    }
    if let Some(voice_handle) = data.voice_handle {
        let mut unassign = true;
        if let Ok((_, mut source)) = query.get_mut(voice_handle.entity) {
            if let Some(voice) = source.voices.get_mut(voice_handle.index) {
                if voice.should_assign {
                    unassign = false;
                    if voice.state_dirty {
                        data.instance_handle = None;
                        match voice.state {
                            AudioPlusVoiceState::Stopped => {
                                channel.stop();
                            }
                            AudioPlusVoiceState::Playing => {
                                channel.stop();
                                if let Some(audio_source) = &voice.audio_source {
                                    data.instance_handle = Some(channel.play(audio_source.clone()));
                                }
                            }
                            AudioPlusVoiceState::Looping => {
                                channel.stop();
                                if let Some(audio_source) = &voice.audio_source {
                                    data.instance_handle =
                                        Some(channel.play_looped(audio_source.clone()));
                                }
                            }
                        }
                        voice.state_dirty = false;
                    }
                    channel.set_volume(voice.volume * voice.volume_multiplier);
                    channel.set_panning(voice.panning);
                    channel.set_playback_rate(voice.playback_rate);
                    if let Some(instance_handle) = &data.instance_handle {
                        let has_position =
                            channel.state(instance_handle.clone()).position().is_some();
                        if voice.status.initialized {
                            voice.status.playing = has_position;
                        } else {
                            voice.status.initialized = has_position;
                            voice.status.playing = true;
                        }
                    }
                }
            }
        }
        if unassign {
            channel.stop();
            channel.set_volume(0.);
            data.voice_handle = None;
            data.instance_handle = None;
        }
    } else {
        let mut found = false;
        for (entity, mut source) in query.iter_mut() {
            for (index, voice) in source.voices.iter_mut().enumerate() {
                if voice.should_assign && !voice.assigned {
                    data.voice_handle = Some(AudioPlusVoiceHandle { entity, index });
                    voice.assigned = true;
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
}

channels!(
    Channel1, Channel2, Channel3, Channel4, Channel5, Channel6, Channel7, Channel8, Channel9,
    Channel10
);
