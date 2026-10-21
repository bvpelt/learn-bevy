use bevy::prelude::*;

// Full-screen dim overlay
pub fn pause_menu_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        justify_content: JustifyContent::Center, // center panel horizontally
        align_items: AlignItems::Center,         // center panel vertically
        ..default()
    }
}

// The centered white panel
pub fn pause_panel_style() -> Node {
    Node {
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(24.0)),
        row_gap: Val::Px(16.0), // even spacing between buttons
        ..default()
    }
}

// A menu button
pub fn pause_button_style() -> Node {
    Node {
        width: Val::Px(220.0),
        height: Val::Px(60.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}

// Button label text
pub fn pause_button_text(label: &str, asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new(label.to_string()),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(28.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

// Title text
pub fn pause_title_text(asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new("Pause Menu"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(48.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.95, 0.95, 0.95);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.85, 0.85, 0.85);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.7, 0.7, 0.7);
