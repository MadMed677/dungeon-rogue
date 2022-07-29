use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ApplicationState, ApplicationStateMenu, ExitTheGameEvent, ResumeTheGameEvent};

use super::components::{
    build_classic_button, build_classic_text, HOVERED_BUTTON, NORMAL_BUTTON, PRESSED_BUTTON,
};

pub struct MainMenuUIPlugin;

#[derive(Component)]
struct MainMenuUI;

enum MenuButtonType {
    Play,
    Load,
    Save,
    Exit,
}

#[derive(Component)]
struct MenuButton(MenuButtonType);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        // Menu container
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(50.0), Val::Percent(95.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        justify_content: JustifyContent::SpaceBetween,
                        border: Rect::all(Val::Px(5.0)),
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    color: Color::rgba(0.2, 0.2, 0.2, 0.5).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                                align_items: AlignItems::Center,
                                justify_content: JustifyContent::Center,
                                ..Default::default()
                            },
                            color: Color::rgba(0.2, 0.2, 0.2, 0.1).into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(build_classic_text(
                                "Dungeon Rogue",
                                &asset_server,
                                None,
                            ));
                        });
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(70.0)),
                                flex_direction: FlexDirection::ColumnReverse,
                                border: Rect::all(Val::Px(5.0)),
                                ..Default::default()
                            },
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(build_classic_button())
                                .insert(MenuButton(MenuButtonType::Play))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Play",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(build_classic_button())
                                .insert(MenuButton(MenuButtonType::Save))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Save",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(build_classic_button())
                                .insert(MenuButton(MenuButtonType::Load))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Load",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        })
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(build_classic_button())
                                .insert(MenuButton(MenuButtonType::Exit))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Exit",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        });
                });
        })
        .insert(MainMenuUI);
}

fn destroy(mut commands: Commands, main_menu_ui_query: Query<Entity, With<MainMenuUI>>) {
    let main_menu_entity = main_menu_ui_query.single();

    commands.entity(main_menu_entity).despawn_recursive();
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&MenuButton>),
        (Changed<Interaction>, With<Button>),
    >,
    mut resume_game_event: EventWriter<ResumeTheGameEvent>,
    mut exit_game_event: EventWriter<ExitTheGameEvent>,
) {
    for (interaction, mut color, menu_button) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = UiColor(PRESSED_BUTTON);
                if let Some(menu_button) = menu_button {
                    match menu_button.0 {
                        MenuButtonType::Play => {
                            resume_game_event.send(ResumeTheGameEvent);
                        }
                        MenuButtonType::Exit => {
                            exit_game_event.send(ExitTheGameEvent);
                        }
                        _ => {}
                    }
                }
            }
            Interaction::Hovered => {
                *color = UiColor(HOVERED_BUTTON);
            }
            Interaction::None => {
                *color = UiColor(NORMAL_BUTTON);
            }
        }
    }
}

impl Plugin for MainMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Menu(ApplicationStateMenu::Main))
                .with_system(button_interaction)
                .into(),
        )
        .add_enter_system(ApplicationState::Menu(ApplicationStateMenu::Main), setup)
        .add_exit_system(ApplicationState::Menu(ApplicationStateMenu::Main), destroy);
    }
}