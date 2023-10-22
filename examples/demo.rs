use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_audio_plus::prelude::*;
use bevy_egui::{
    egui::{self, Pos2},
    EguiContexts, EguiPlugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Audio Plus - Demo".to_string(),
                resolution: WindowResolution::new(640., 540.),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((EguiPlugin, AudioPlusPlugin))
        .add_systems(Startup, init)
        .add_systems(Update, ui_example)
        .run();
}

#[derive(Component)]
pub struct Label(pub String);

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle::default(), AudioPlusListener));
    commands.spawn((
        SpriteBundle::default(),
        Label("Rock".into()),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/rock.ogg"))
                .with_channel(AudioPlusMixerChannel::Sfx),
        ),
    ));
    commands.spawn((
        SpriteBundle::default(),
        Label("Music".into()),
        AudioPlusSource::new(
            AudioPlusSoundEffect::single(asset_server.load("sounds/music_1.ogg"))
                .with_channel(AudioPlusMixerChannel::Music),
        ),
    ));
}

fn ui_example(
    mut contexts: EguiContexts,
    mut query: Query<(&Label, &mut AudioPlusSource, &mut Transform)>,
    mut mixer: ResMut<AudioPlusMixer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let (width, height) = if let Ok(window) = window_query.get_single() {
        (window.width(), window.height())
    } else {
        (640., 480.)
    };
    let padding = 16.;
    egui::Window::new("Hello")
        .frame(egui::containers::Frame::none())
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .vscroll(true)
        .fixed_pos(Pos2::new(padding, padding))
        .min_height(height - padding * 2.)
        .show(contexts.ctx_mut(), |ui| {
            ui.set_width(width - padding * 2.);

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Mixer");
                    ui.group(|ui| {
                        ui.label("SFX");
                        let mut sfx_volume = mixer.get_volume(AudioPlusMixerChannel::Sfx);
                        ui.add(egui::Slider::new(&mut sfx_volume, 0.0..=1.));
                        mixer.set_volume(AudioPlusMixerChannel::Sfx, sfx_volume);

                        ui.label("Music");
                        let mut music_volume = mixer.get_volume(AudioPlusMixerChannel::Music);
                        ui.add(egui::Slider::new(&mut music_volume, 0.0..=1.));
                        mixer.set_volume(AudioPlusMixerChannel::Music, music_volume);
                    });
                });

                for (i, (label, mut source, mut transform)) in query.iter_mut().enumerate() {
                    ui.vertical(|ui| {
                        ui.label(&label.0);
                        ui.group(|ui| {
                            {
                                let sound_effect = source.effect_mut();

                                let mut voices = sound_effect.voices();
                                ui.label("Voices");
                                ui.add(egui::Slider::new(&mut voices, 1..=20));
                                sound_effect.set_voices(voices);

                                let mut volume = sound_effect.volume();
                                ui.label("Volume");
                                ui.add(egui::Slider::new(&mut volume, 0.0..=1.));
                                sound_effect.set_volume(volume);

                                let mut volume_variation = sound_effect.volume_variation();
                                ui.label("Volume Variation");
                                ui.add(egui::Slider::new(&mut volume_variation, 0.0..=1.));
                                sound_effect.set_volume_variation(volume_variation);

                                let mut pitch = sound_effect.pitch();
                                ui.label("Pitch");
                                ui.add(egui::Slider::new(&mut pitch, 0.0..=2.));
                                sound_effect.set_pitch(pitch);

                                let mut pitch_variation = sound_effect.pitch_variation();
                                ui.label("Pitch Variation");
                                ui.add(egui::Slider::new(&mut pitch_variation, 0.0..=1.));
                                sound_effect.set_pitch_variation(pitch_variation);

                                let mut chance = sound_effect.chance();
                                ui.label("Chance");
                                ui.add(egui::Slider::new(&mut chance, 0.0..=1.));
                                sound_effect.set_chance(chance);

                                let mut fade_in = sound_effect.fade_in();
                                ui.label("Fade In");
                                ui.add(egui::Slider::new(&mut fade_in, 0.0..=5.));
                                sound_effect.set_fade_in(fade_in);

                                let mut fade_out = sound_effect.fade_out();
                                ui.label("Fade Out");
                                ui.add(egui::Slider::new(&mut fade_out, 0.0..=5.));
                                sound_effect.set_fade_out(fade_out);

                                let mut channel = sound_effect.channel();
                                ui.label("Channel");
                                egui::ComboBox::new(format!("Channel {}", i), "")
                                    .selected_text(format!("{:?}", channel))
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(
                                            &mut channel,
                                            AudioPlusMixerChannel::None,
                                            "None",
                                        );
                                        ui.selectable_value(
                                            &mut channel,
                                            AudioPlusMixerChannel::Music,
                                            "Music",
                                        );
                                        ui.selectable_value(
                                            &mut channel,
                                            AudioPlusMixerChannel::Sfx,
                                            "Sfx",
                                        );
                                    });
                                sound_effect.set_channel(channel);
                            }

                            ui.horizontal(|ui| {
                                if ui.button("Play").clicked() {
                                    source.play();
                                }
                                if ui.button("Loop").clicked() {
                                    source.play_looped();
                                }
                                if ui.button("Stop").clicked() {
                                    source.stop();
                                }
                            });

                            {
                                let sound_effect = source.effect_mut();
                                let mut positional = sound_effect.positional();
                                ui.checkbox(&mut positional, "Positional");
                                sound_effect.set_positional(positional);
                                if positional {
                                    let mut distance = sound_effect.distance();
                                    ui.label("Falloff Distance");
                                    ui.add(egui::Slider::new(&mut distance, 1.0..=5000.));
                                    sound_effect.set_distance(distance);

                                    let mut x = transform.translation.x;
                                    ui.label("X");
                                    ui.add(egui::Slider::new(&mut x, -1000.0..=1000.));
                                    transform.translation.x = x;
                                }
                            }
                        });
                    });
                }
            });
        });
}
