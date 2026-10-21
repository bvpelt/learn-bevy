use bevy::prelude::*;

use crate::game::ui::game_over_menu::components::*;
use crate::game::ui::game_over_menu::styles::*;

pub fn spawn_game_over_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _menu_entity = build_game_over_menu(&mut commands, &asset_server);
}

pub fn despawn_game_over_menu(
    mut commands: Commands,
    menu_query: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(menu_entity) = menu_query.single() {
        commands.entity(menu_entity).despawn();
    }
}

pub fn build_game_over_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            game_over_menu_style(),
            BackgroundColor(Color::srgb(0.4, 0.4, 0.4).with_alpha(0.8)),
            GameOverMenu,
        ))
        .with_children(|overlay| {
            overlay
                .spawn((
                    game_over_panel_style(),
                    BackgroundColor(Color::srgb(0.97, 0.97, 0.97)),
                    BorderColor(Color::srgb(0.7, 0.7, 0.7)),
                    BorderRadius::all(Val::Px(12.0)),
                ))
                .with_children(|panel| {
                    // Title
                    panel.spawn(game_over_title_text(asset_server));

                    // Final score
                    panel.spawn((final_score_text(asset_server), FinalScoreText));

                    // Restart
                    panel
                        .spawn((
                            game_over_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            RestartButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(game_over_button_text("Restart", asset_server));
                        });

                    // Main Menu
                    panel
                        .spawn((
                            game_over_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            MainMenuButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(game_over_button_text("Main Menu", asset_server));
                        });

                    // Quit
                    panel
                        .spawn((
                            game_over_button_style(),
                            BackgroundColor(NORMAL_BUTTON_COLOR),
                            Interaction::None,
                            QuitButton,
                        ))
                        .with_children(|btn| {
                            btn.spawn(game_over_button_text("Quit", asset_server));
                        });
                });
        })
        .id()
}
