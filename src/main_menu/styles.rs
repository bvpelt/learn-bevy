use bevy::prelude::*;

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);

// Function instead of const — `..default()` is a runtime call
pub fn button_style() -> Node {
    Node {
        width: Val::Px(200.0),
        height: Val::Px(80.0),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}
