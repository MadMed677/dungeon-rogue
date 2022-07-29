use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{ApplicationState, ApplicationStateMenu};

use super::components::{build_classic_button, build_classic_text, ClassicButtonTextParams};

pub struct DeadMenuUIPlugin;

enum DeadButtonType {
    Play,
    Exit,
}

#[derive(Component)]
struct DeadMenuUI;

#[derive(Component)]
struct MenuButton(DeadButtonType);

impl Plugin for DeadMenuUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Menu(ApplicationStateMenu::Dead))
                .with_system(button_interaction)
                .into(),
        )
        .add_enter_system(ApplicationState::Menu(ApplicationStateMenu::Dead), setup)
        .add_exit_system(ApplicationState::Menu(ApplicationStateMenu::Dead), destroy);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
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
                                "You are dead",
                                &asset_server,
                                Some(ClassicButtonTextParams { font_size: 30.0 }),
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
                                .insert(MenuButton(DeadButtonType::Play))
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
                                .insert(MenuButton(DeadButtonType::Exit))
                                .with_children(|parent| {
                                    parent.spawn_bundle(build_classic_text(
                                        "Save",
                                        &asset_server,
                                        None,
                                    ));
                                });
                        });
                });
        })
        .insert(DeadMenuUI);
}

fn destroy(mut commands: Commands, dead_menu_ui_query: Query<Entity, With<DeadMenuUI>>) {
    let dead_menu_entity = dead_menu_ui_query.single();

    commands.entity(dead_menu_entity).despawn_recursive();
}

fn button_interaction() {}
