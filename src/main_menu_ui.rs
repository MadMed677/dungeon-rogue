use bevy::prelude::*;

use crate::{ApplicationState, ResumeTheGameEvent};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub struct MainMenuUIPlugin;

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct SimulationClickEvent;

fn build_classic_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            margin: Rect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: UiColor(NORMAL_BUTTON),
        ..Default::default()
    }
}

fn build_classic_text(value: &str, asset_server: &Res<AssetServer>) -> TextBundle {
    TextBundle {
        text: Text::with_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            Default::default(),
        ),
        ..Default::default()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                        border: Rect::all(Val::Px(5.0)),
                        ..Default::default()
                    },
                    color: Color::rgb(0.1, 0.1, 0.1).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(build_classic_button())
                        .with_children(|parent| {
                            parent.spawn_bundle(build_classic_text("Play", &asset_server));
                        });
                });
        })
        .insert(MainMenuUI);
}

fn destroy(mut commands: Commands, main_menu_ui_query: Query<Entity, With<MainMenuUI>>) {
    if let Ok(main_menu_entity) = main_menu_ui_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn button_interaction(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut resume_game_event: EventWriter<ResumeTheGameEvent>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = UiColor(PRESSED_BUTTON);
                resume_game_event.send(ResumeTheGameEvent);
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
        app.add_event::<SimulationClickEvent>()
            .add_system_set(SystemSet::on_enter(ApplicationState::Menu).with_system(setup))
            .add_system_set(SystemSet::on_exit(ApplicationState::Menu).with_system(destroy))
            .add_system(button_interaction);
    }
}
