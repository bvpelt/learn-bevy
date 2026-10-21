use bevy::prelude::*;

use crate::main_menu::ui::pause_menu::components::*;
use crate::main_menu::ui::pause_menu::styles::*;

pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _pause_menu_entity = build_pause_menu(&mut commands, &asset_server);
}

pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.single() {
        commands.entity(pause_menu_entity).despawn();
    }
}

pub fn build_pause_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            pause_menu_style(),
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4).with_alpha(0.8)), // dim overlay
            PauseMenu {},
        ))
        .with_children(|overlay| {
            // Centered white panel
            overlay
                .spawn((
                    pause_panel_style(),
                    BackgroundColor(Color::srgb(0.97, 0.97, 0.97)),
                    BorderColor::all(Color::srgb(0.7, 0.7, 0.7)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn(pause_title_text(asset_server));

                    // Resume
                    panel
                        .spawn((
                            pause_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            ResumeButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(pause_button_text("Resume", asset_server));
                        });

                    // Main Menu
                    panel
                        .spawn((
                            pause_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            MainButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(pause_button_text("Main Menu", asset_server));
                        });

                    // Quit
                    panel
                        .spawn((
                            pause_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            QuitButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(pause_button_text("Quit", asset_server));
                        });
                });
        })
        .id()
}
