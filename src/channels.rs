use crate::{source::AudioPlusSource, AudioPlusSystems};
use bevy::ecs::system::Resource;
use bevy::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel};

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
    entity: Option<Entity>,
    last_state: ChannelState,
}

#[derive(Default, Clone)]
pub(crate) struct ChannelSettings {
    pub(crate) should_assign: bool,
    pub(crate) assigned: bool,
    pub(crate) volume: f32,
    pub(crate) panning: f32,
    pub(crate) state: ChannelState,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ChannelState {
    #[default]
    Stopped,
    Playing,
    Looping,
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
    if let Some(entity) = data.entity {
        let mut unassign = true;
        if let Ok((_, source)) = query.get(entity) {
            if source.channel_settings.should_assign {
                unassign = false;
                if data.last_state != source.channel_settings.state {
                    match source.channel_settings.state {
                        ChannelState::Stopped => {
                            channel.stop();
                        }
                        ChannelState::Playing => {
                            channel.stop();
                            channel.play(source.resource.clone());
                        }
                        ChannelState::Looping => {
                            channel.stop();
                            channel.play_looped(source.resource.clone());
                        }
                    }
                    data.last_state = source.channel_settings.state;
                }
                channel.set_volume(source.channel_settings.volume);
                channel.set_panning(source.channel_settings.panning);
            }
        }
        if unassign {
            channel.stop();
            channel.set_volume(0.);
            data.entity = None;
            data.last_state = ChannelState::Stopped;
        }
    } else {
        for (entity, mut source) in query.iter_mut() {
            if source.channel_settings.should_assign && !source.channel_settings.assigned {
                data.entity = Some(entity);
                source.channel_settings.assigned = true;
                break;
            }
        }
    }
}

channels!(
    Channel1, Channel2, Channel3, Channel4, Channel5, Channel6, Channel7, Channel8, Channel9,
    Channel10
);
