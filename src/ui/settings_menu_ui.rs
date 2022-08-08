use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use iyes_loopless::prelude::*;

use crate::{audio::AudioState, ApplicationState, ApplicationStateMenu};

use super::components::{
    build_classic_button, build_classic_text, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
};

pub struct SettingsMenuUIPlugin;

impl Plugin for SettingsMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Menu(ApplicationStateMenu::Settings))
                .with_system(button_interaction)
                .with_system(update_buttons_color)
                .with_system(change_music_state)
                .with_system(change_music_volume)
                .into(),
        )
        .add_enter_system(
            ApplicationState::Menu(ApplicationStateMenu::Settings),
            setup,
        )
        .add_exit_system(
            ApplicationState::Menu(ApplicationStateMenu::Settings),
            destroy,
        )
        .add_event::<ChangeMusicStateEvent>()
        .add_event::<ChangeMusicVolumeEvent>();
    }
}

#[derive(Component)]
struct SettingsMenuUI;

#[derive(Debug, Inspectable)]
enum MusicState {
    On,
    Off,
}

impl Default for MusicState {
    fn default() -> Self {
        Self::Off
    }
}

#[derive(Debug, Inspectable)]
enum SettingsButtonType {
    MusicStatus(MusicState),
    MusicVolume(i8),
    BackToMenu,
}

#[derive(Component, Inspectable)]
pub struct SettingsButton(SettingsButtonType);

#[derive(Component)]
struct Active;

#[derive(Debug, Inspectable, Component)]
enum SettingsTextType {
    Volume,
}

/// Event which triggers when the music state
///  should be changed
/// Accepts Entity on which state user select state
struct ChangeMusicStateEvent(Entity);

/// Event which triggers when the music volume
///  should be changed
/// Accepts the diff on which we should change the volume
struct ChangeMusicVolumeEvent(i8);

#[allow(clippy::type_complexity)]
fn button_interaction(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut UiColor,
            &SettingsButton,
            Option<&Active>,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut change_music_state_event: EventWriter<ChangeMusicStateEvent>,
    mut change_music_volume_event: EventWriter<ChangeMusicVolumeEvent>,
) {
    for (entity, interaction, mut color, settings_button, active) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => match &settings_button.0 {
                SettingsButtonType::MusicStatus(_) => {
                    change_music_state_event.send(ChangeMusicStateEvent(entity));
                }
                SettingsButtonType::MusicVolume(volume) => {
                    change_music_volume_event.send(ChangeMusicVolumeEvent(*volume));
                }
                SettingsButtonType::BackToMenu => {
                    commands.insert_resource(NextState(ApplicationState::Menu(
                        ApplicationStateMenu::Main,
                    )));
                }
            },
            Interaction::Hovered => {
                if active.is_none() {
                    *color = UiColor(HOVERED_BUTTON);
                }
            }
            Interaction::None => {
                if active.is_none() {
                    *color = UiColor(NORMAL_BUTTON);
                }
            }
        }
    }
}

fn update_buttons_color(
    mut music_buttons_query: Query<
        (&mut UiColor, &Interaction, Option<&Active>),
        With<SettingsButton>,
    >,
) {
    for (mut color, interaction, active) in music_buttons_query.iter_mut() {
        if active.is_some() {
            *color = UiColor(PRESSED_BUTTON);
        } else {
            if Interaction::Hovered == *interaction {
                return;
            }

            *color = UiColor(NORMAL_BUTTON);
        }
    }
}

fn change_music_volume(
    mut change_music_volume_event: EventReader<ChangeMusicVolumeEvent>,
    mut audio_state: ResMut<AudioState>,
    mut music_text_query: Query<(&mut Text, &SettingsTextType), With<SettingsTextType>>,
) {
    for event in change_music_volume_event.iter() {
        if (event.0 == -1 && audio_state.volume != 0) || (event.0 == 1 && audio_state.volume != 10)
        {
            audio_state.volume += event.0;
        }

        for (mut music_text_text, _) in music_text_query.iter_mut() {
            // We should check the Volume text type when we will have more than
            //  one enum value
            // if let SettingsTextType::Volume = &music_text_type {
            // }

            music_text_text.sections[0].value = audio_state.volume.to_string();
        }
    }
}

fn change_music_state(
    mut commands: Commands,
    music_buttons_query: Query<(Entity, &SettingsButton), With<SettingsButton>>,
    mut change_music_state_event: EventReader<ChangeMusicStateEvent>,
    active_button_query: Query<&SettingsButton>,
    mut audio_state: ResMut<AudioState>,
) {
    for event in change_music_state_event.iter() {
        for (music_button_entity, music_button_settings) in music_buttons_query.iter() {
            if let SettingsButtonType::MusicStatus(_) = &music_button_settings.0 {
                commands.entity(music_button_entity).remove::<Active>();
            }
        }

        let active_button = event.0;

        commands.entity(active_button).insert(Active);

        if let Ok(active_button_settings) = active_button_query.get(active_button) {
            if let SettingsButtonType::MusicStatus(music_state) = &active_button_settings.0 {
                // Turn the music on / off
                match music_state {
                    MusicState::On => {
                        audio_state.state = true;
                    }
                    MusicState::Off => {
                        audio_state.state = false;
                    }
                }
            }
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio_state: Res<AudioState>) {
    commands
        // Top-level container which contains the whole page
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(20.0), Val::Percent(20.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0.2, 0.2, 0.2, 0.1).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(build_classic_text("Settings", &asset_server, None));
                });
        })
        .with_children(|parent| {
            // Right panel
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(45.0), Val::Percent(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::SpaceBetween,
                        align_content: AlignContent::Center,
                        align_items: AlignItems::Center,
                        border: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                // Spawn music block
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(30.0)),
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        margin: Rect::all(Val::Px(30.0)),
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Music",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    let mut on_button = parent.spawn_bundle(build_classic_button());

                                    on_button
                                        .insert(SettingsButton(SettingsButtonType::MusicStatus(
                                            MusicState::On,
                                        )))
                                        .with_children(|parent| {
                                            parent.spawn_bundle(build_classic_text(
                                                "On",
                                                &asset_server,
                                                None,
                                            ));
                                        });

                                    if audio_state.state {
                                        on_button.insert(Active);
                                    }

                                    let mut off_button =
                                        parent.spawn_bundle(build_classic_button());

                                    off_button
                                        .insert(SettingsButton(SettingsButtonType::MusicStatus(
                                            MusicState::Off,
                                        )))
                                        .with_children(|parent| {
                                            parent.spawn_bundle(build_classic_text(
                                                "Off",
                                                &asset_server,
                                                None,
                                            ));
                                        });

                                    if !audio_state.state {
                                        off_button.insert(Active);
                                    }
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                        justify_content: JustifyContent::Center,
                                        align_items: AlignItems::Center,
                                        ..Default::default()
                                    },
                                    color: Color::NONE.into(),
                                    ..Default::default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(build_classic_button())
                                        .insert(SettingsButton(SettingsButtonType::MusicVolume(-1)))
                                        .with_children(|parent| {
                                            parent.spawn_bundle(build_classic_text(
                                                "-1",
                                                &asset_server,
                                                None,
                                            ));
                                        });

                                    parent
                                        .spawn_bundle(build_classic_text(
                                            audio_state.volume.to_string().as_str(),
                                            &asset_server,
                                            None,
                                        ))
                                        .insert(SettingsTextType::Volume);

                                    parent
                                        .spawn_bundle(build_classic_button())
                                        .insert(SettingsButton(SettingsButtonType::MusicVolume(1)))
                                        .with_children(|parent| {
                                            parent.spawn_bundle(build_classic_text(
                                                "+1",
                                                &asset_server,
                                                None,
                                            ));
                                        });
                                });
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                                justify_content: JustifyContent::Center,
                                flex_direction: FlexDirection::ColumnReverse,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            color: Color::rgba(0.1, 0.1, 0.1, 0.8).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(build_classic_button())
                                .insert(SettingsButton(SettingsButtonType::BackToMenu))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Back",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        });
                });
        })
        .insert(SettingsMenuUI);
}

fn destroy(mut commands: Commands, settings_menu_ui_query: Query<Entity, With<SettingsMenuUI>>) {
    let settings_menu_ui = settings_menu_ui_query.single();

    commands.entity(settings_menu_ui).despawn_recursive();
}
