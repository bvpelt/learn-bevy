use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

// Function instead of const — `..default()` is a runtime call
pub fn main_menu_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        row_gap: Val::Px(8.0), // or UiValue::Px(8.0) for one axis
        ..default()
    }
}

pub fn button_style() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

pub fn title_button_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        width: Val::Px(500.0),
        height: Val::Px(120.0),
        ..default()
    }
}

pub fn image_style() -> Node {
    Node {
        width: Val::Px(64.0),
        height: Val::Px(64.0),
        margin: UiRect {
            left: Val::Px(8.0),
            right: Val::Px(8.0),
            top: Val::Px(8.0),
            bottom: Val::Px(8.0),
        },
        ..default()
    }
}

//pub fn text_style(font_size: f32, asset_server: &AssetServer) -> (TextFont, TextColor, TextLayout) {
pub fn text_style(
    font_size: f32,
    asset_server: &Res<AssetServer>,
) -> (TextFont, TextColor, TextLayout) {
    (
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(font_size),
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout {
            justify: Justify::Center,
            ..default()
        },
    )
}
