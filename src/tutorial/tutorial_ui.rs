use bevy::prelude::*;
use bevy_ecs_ldtk::LevelSelection;
use iyes_loopless::prelude::*;

use crate::tutorial::tutorial_physics::{Tutorial, TutorialPassed, TutorialType};
use crate::{ApplicationState, Sprites};

#[derive(Component)]
struct MovementTutorialUi;

pub struct TutorialUiPlugin;

fn spawn_movement_ui(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    sprites: &Res<Sprites>,
    tutorial_type: &TutorialType,
) -> Entity {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(50.0),
                    top: Val::Px(50.0),
                    ..Default::default()
                },
                size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            color: Color::rgba(0.5, 0.5, 0.5, 0.2).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // Render a div to place the text
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            match tutorial_type {
                                TutorialType::Movement => "Movement",
                                TutorialType::Climbing => "Climbing",
                            },
                            TextStyle {
                                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                            Default::default(),
                        ),
                        ..Default::default()
                    });
                });

            // Render a div to place the keyboard movement image
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(60.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    // color: Color::rgb(0.0, 1.0, 0.0).into(),
                    color: Color::NONE.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Percent(90.0)),
                            ..Default::default()
                        },
                        image: sprites.tutorial_movement.clone().into(),
                        ..Default::default()
                    });
                });
        })
        .insert(MovementTutorialUi)
        .id()
}

fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    level_selection: Res<LevelSelection>,
    mut tutorial_query: Query<
        (&TutorialType, &mut Tutorial, &TutorialPassed),
        Changed<TutorialPassed>,
    >,
) {
    if let LevelSelection::Uid(current_level) = level_selection.as_ref() {
        // We would like to place movement tutorial only on the first level
        if *current_level != 0 {
            return;
        }

        for (tutorial_type, mut tutorial, tutorial_triggered) in tutorial_query.iter_mut() {
            // If tutorial has been triggered let's return
            if tutorial_triggered.0 == false {
                return;
            }

            // Create a tutorial UI
            let movement_tutorial_ui =
                spawn_movement_ui(&mut commands, &asset_server, &sprites, &tutorial_type);

            tutorial.ui_entities.insert(movement_tutorial_ui);
        }
    }
}

fn destroy(
    mut commands: Commands,
    movement_tutorial_ui_query: Query<Entity, With<MovementTutorialUi>>,
) {
    if let Ok(movement_tutorial_entity) = movement_tutorial_ui_query.get_single() {
        commands
            .entity(movement_tutorial_entity)
            .despawn_recursive();
    }
}

impl Plugin for TutorialUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(setup)
                // .with_system(change_tutorial_visibility)
                .into(),
        )
        // .add_enter_system(ApplicationState::Game, setup)
        .add_exit_system(ApplicationState::Game, destroy);
    }
}
