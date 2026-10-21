use bevy::prelude::*;

use crate::game::HUD_HEIGHT;

pub fn hud_menu_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Px(HUD_HEIGHT),
        justify_content: JustifyContent::SpaceBetween, // pushes left & right groups apart
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        ..default()
    }
}

// A group: [image][number] or [number][image], laid out horizontally
pub fn hud_group_style() -> Node {
    Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        column_gap: Val::Px(8.0),
        ..default()
    }
}

// Small wrapper for an icon image
pub fn hud_icon_style() -> Node {
    Node {
        width: Val::Px(32.0),
        height: Val::Px(32.0),
        ..default()
    }
}

pub fn hud_text_style(asset_server: &AssetServer) -> (Text, TextFont, TextColor) {
    (
        Text::new("0"),
        TextFont {
            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
            font_size: FontSize::Px(32.0),
            ..default()
        },
        TextColor(Color::srgb(0.1, 0.1, 0.1)),
    )
}
