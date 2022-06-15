use bevy::{prelude::*, render::camera::Camera2d};
use bevy_ecs_ldtk::LevelSelection;
use iyes_loopless::prelude::*;

use crate::{
    tutorial::{Tutorial, TutorialType},
    ApplicationState, Sprites,
};

#[derive(Component)]
struct MovementTutorialUi;

pub struct MovementTutorialUiPlugin;

fn setup(
    mut commands: Commands,
    sprites: Res<Sprites>,
    asset_server: Res<AssetServer>,
    level_selection: Res<LevelSelection>,
    mut tutorial_query: Query<(Entity, &TutorialType, &Transform, &mut Tutorial), Added<Tutorial>>,
) {
    if let LevelSelection::Uid(current_level) = level_selection.as_ref() {
        // We would like to place movement tutorial only on the first level
        if *current_level != 0 {
            return;
        }

        for (tutorial_entity, tutorial_type, transform, mut tutorial) in tutorial_query.iter_mut() {
            println!("tutorial_type: {:?}", tutorial_type);
            println!("transform: {:?}", transform.translation);

            // Create a tutorial UI
            let ui_entity = commands
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        position: Rect {
                            left: Val::Px(transform.translation.x * 2.0),
                            bottom: Val::Px(transform.translation.y * 2.0),
                            ..Default::default()
                        },
                        // position: Rect {
                        //     left: Val::Px(250.0),
                        //     bottom: Val::Px(250.0),
                        //     ..Default::default()
                        // },
                        size: Size::new(Val::Px(100.0), Val::Px(100.0)),
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    transform: *transform,
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
                            // color: Color::rgb(1.0, 0.0, 0.0).into(),
                            color: Color::NONE.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            parent.spawn_bundle(TextBundle {
                                text: Text::with_section(
                                    "Movement",
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
                                // transform: Transform::from_xyz(200.0, 400.0, 2.0),
                                ..Default::default()
                            });
                        });
                })
                .insert(MovementTutorialUi)
                .id();

            tutorial.ui_entities.insert(ui_entity);
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

impl Plugin for MovementTutorialUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                // .with_system(shift_movement_tutorial)
                .into(),
        )
        .add_enter_system(ApplicationState::Game, setup)
        .add_exit_system(ApplicationState::Game, destroy);
    }
}
