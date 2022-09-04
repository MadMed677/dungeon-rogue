use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{common::Health, player::Player, ApplicationState};

/// Show the Heads-up Display for the entities who have a Health component
pub struct HudPlugin;

#[derive(Component)]
struct HudCurrentHealthUI;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(ApplicationState::Game)
                .with_system(spawn_enemies_hud_ui)
                .with_system(update_player_hud_ui)
                .with_system(update_enemies_hud_ui)
                .into(),
        )
        .add_enter_system(ApplicationState::Game, spawn_player_hud_ui)
        .add_exit_system(ApplicationState::Game, despawn_player_hud_ui);

        // Save all parameters to use it in `update_hud_ui` system and others
        app.insert_resource(HudResourse {
            health_ui_width_px: 20.0,
            health_ui_height_px: 2.5,
            health_ui_margin_top_px: 15.0,
        });
    }
}

struct HudResourse {
    health_ui_width_px: f32,
    health_ui_height_px: f32,
    health_ui_margin_top_px: f32,
}

#[derive(Component)]
struct PlayerHud;

#[derive(Component)]
struct PlayerTextHud;

#[derive(Component)]
struct PlayerHealthBarHud;

/// Current HuD has different sizes for each cell. Based on that
///  we have to hard-code values based on specific health state
///  this method works with health values: 0, 1, 2, 3, 4, 5
///  where 5 - is a full health bar, 0 - player is dead
fn calculate_player_hud_shift(health: &Health) -> UiRect<Val> {
    let current_in_percent = health.current * 100 / health.max;
    let x_shift = match 100 - current_in_percent {
        0 => 0.0,
        20 => -22.0,
        40 => -38.0,
        60 => -55.0,
        80 => -72.0,
        100 => -100.0,
        _ => 0.0,
    };

    UiRect::new(
        Val::Percent(x_shift),
        Val::Px(0.0),
        Val::Px(0.0),
        Val::Px(0.0),
    )
}

fn spawn_player_hud_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_health: Query<&Health, With<Player>>,
) {
    if let Ok(health) = player_health.get_single() {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Px(84.0), Val::Px(48.0)),
                    flex_direction: FlexDirection::ColumnReverse,
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::SpaceBetween,
                    position: UiRect::new(
                        Val::Percent(7.0),
                        Val::Auto,
                        Val::Percent(8.0),
                        Val::Auto,
                    ),
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::FlexStart,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn_bundle(ImageBundle {
                            image: asset_server.load("atlas/player/HUD-life-count.png").into(),
                            ..Default::default()
                        });

                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    size: Size::new(Val::Auto, Val::Auto),
                                    align_items: AlignItems::FlexStart,
                                    margin: UiRect::new(
                                        Val::Px(5.0),
                                        Val::Px(0.0),
                                        Val::Px(0.0),
                                        Val::Px(0.0),
                                    ),
                                    ..Default::default()
                                },
                                color: Color::NONE.into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle::from_section(
                                        format!("0{}", health.current),
                                        TextStyle {
                                            font: asset_server
                                                .load("fonts/NicoPaint-Monospaced.ttf"),
                                            font_size: 22.0,
                                            color: Color::WHITE,
                                        },
                                    ))
                                    .insert(PlayerTextHud);
                            });
                    });

                parent
                    .spawn_bundle(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            ..Default::default()
                        },
                        color: Color::NONE.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(ImageBundle {
                                style: Style {
                                    size: Size::new(Val::Px(84.0), Val::Auto),
                                    ..Default::default()
                                },
                                image: asset_server
                                    .load("atlas/player/HUD-life-bar-container.png")
                                    .into(),
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(NodeBundle {
                                        style: Style {
                                            size: Size::new(
                                                Val::Percent(100.0),
                                                Val::Percent(100.0),
                                            ),
                                            overflow: Overflow::Hidden,
                                            ..Default::default()
                                        },
                                        color: Color::NONE.into(),
                                        ..Default::default()
                                    })
                                    .with_children(|parent| {
                                        parent
                                            .spawn_bundle(ImageBundle {
                                                style: Style {
                                                    size: Size::new(
                                                        Val::Percent(100.0),
                                                        Val::Percent(100.0),
                                                    ),
                                                    position: calculate_player_hud_shift(health),
                                                    ..Default::default()
                                                },
                                                image: asset_server
                                                    .load("atlas/player/HUD-life-bar.png")
                                                    .into(),
                                                ..Default::default()
                                            })
                                            .insert(PlayerHealthBarHud);
                                    });
                            });
                    });
            })
            .insert(PlayerHud);
    }
}

fn despawn_player_hud_ui(mut commands: Commands, player_hud: Query<Entity, With<PlayerHud>>) {
    if let Ok(hud) = player_hud.get_single() {
        commands.entity(hud).despawn_recursive();
    }
}

/// Spawn HuD (Heads-up Display) above the enemies which has more `Health` component
fn spawn_enemies_hud_ui(
    mut commands: Commands,
    health_query: Query<(Entity, &Health), (Added<Health>, Without<Player>)>,
    hud_resource: Res<HudResourse>,
) {
    for (health_entity, health) in health_query.iter() {
        let pixel_per_point = hud_resource.health_ui_width_px / health.max as f32;

        let health_current_width_px = pixel_per_point * health.current as f32;

        // We have to make a translation by X axis because current health bar
        //  centralized by center-center, but we have to make start current health
        //  bar with the same coordinate as the first bar began.
        // To do so we have to calculate the difference between them and devide by 2
        //  becase we have the same gap from left and right but we have to translate
        //  it only from one side
        let current_health_translation_px =
            (hud_resource.health_ui_width_px - health_current_width_px) / 2.0;

        commands.entity(health_entity).with_children(|parent| {
            parent
                .spawn_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.0, 1.0, 0.0, 0.7),
                        custom_size: Some(Vec2::new(
                            health_current_width_px,
                            hud_resource.health_ui_height_px,
                        )),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        -current_health_translation_px,
                        hud_resource.health_ui_margin_top_px,
                        1.0,
                    ),
                    ..Default::default()
                })
                .insert(HudCurrentHealthUI);

            parent.spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.0, 0.0, 0.3),
                    custom_size: Some(Vec2::new(
                        hud_resource.health_ui_width_px,
                        hud_resource.health_ui_height_px,
                    )),
                    ..Default::default()
                },
                transform: Transform::from_xyz(0.0, hud_resource.health_ui_margin_top_px, 1.0),
                ..Default::default()
            });
        });
    }
}

fn update_player_hud_ui(
    health_query: Query<&Health, (Changed<Health>, With<Player>)>,
    mut health_ui_text_query: Query<&mut Text, With<PlayerTextHud>>,
    mut health_ui_bar_query: Query<&mut Style, With<PlayerHealthBarHud>>,
) {
    for health in health_query.iter() {
        for mut text in health_ui_text_query.iter_mut() {
            text.sections[0].value = format!("0{}", health.current);

            if health.current <= 2 {
                text.sections[0].style.color = Color::Rgba {
                    red: 1.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 1.0,
                };
            } else {
                text.sections[0].style.color = Color::WHITE;
            }
        }

        for mut bar in health_ui_bar_query.iter_mut() {
            bar.position = calculate_player_hud_shift(health);
        }
    }
}

/// Update HuD (Heads-up Display) when an entity got damage
/// and `Health` component changed
fn update_enemies_hud_ui(
    health_query: Query<(Entity, &Health, &Children), Changed<Health>>,
    mut health_ui_query: Query<(&mut Sprite, &mut Transform), With<HudCurrentHealthUI>>,
    hud_resource: Res<HudResourse>,
) {
    for (_, health, health_children) in health_query.iter() {
        for &health_child in health_children.iter() {
            if let Ok((mut health_ui_sprite, mut health_ui_transform)) =
                health_ui_query.get_mut(health_child)
            {
                let pixel_per_point = hud_resource.health_ui_width_px / health.max as f32;

                let health_current_width_px = pixel_per_point * health.current as f32;
                let current_health_translation_px =
                    (hud_resource.health_ui_width_px - health_current_width_px) / 2.0;

                health_ui_sprite.custom_size = Some(Vec2::new(
                    health_current_width_px,
                    hud_resource.health_ui_height_px,
                ));
                health_ui_transform.translation = Vec3::new(
                    -current_health_translation_px,
                    hud_resource.health_ui_margin_top_px,
                    1.0,
                );
            }
        }
    }
}
