use bevy::prelude::*;

pub const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn build_classic_button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Px(150.0), Val::Px(50.0)),
            margin: UiRect::all(Val::Auto),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: UiColor(NORMAL_BUTTON),
        ..Default::default()
    }
}

pub struct ClassicButtonTextParams {
    pub font_size: f32,
}

pub fn build_classic_text(
    value: &str,
    asset_server: &Res<AssetServer>,
    params: Option<ClassicButtonTextParams>,
) -> TextBundle {
    let button_params = params.unwrap_or(ClassicButtonTextParams { font_size: 16.0 });

    TextBundle {
        text: Text::from_section(
            value,
            TextStyle {
                font: asset_server.load("fonts/NicoPaint-Regular.ttf"),
                font_size: button_params.font_size,
                color: Color::WHITE,
            },
        ),
        ..Default::default()
    }
}
