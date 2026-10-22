use bevy::prelude::*;

use super::components::*;
use crate::AppState;
use crate::game::score::resources::HighScores;
use crate::main_menu::styles::*;

// widths of the columns: rank, player, level, score
const COLUMN_WIDTHS: [f32; 4] = [60.0, 260.0, 100.0, 120.0];

pub fn spawn_scoreboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    high_scores: Res<HighScores>,
) {
    // header followed by one row per score
    let mut rows: Vec<[String; 4]> = vec![["#", "Player", "Level", "Score"].map(String::from)];
    for (index, entry) in high_scores.scores.iter().enumerate() {
        rows.push([
            (index + 1).to_string(),
            entry.player_name.clone(),
            entry.level.to_string(),
            entry.score.to_string(),
        ]);
    }

    commands
        .spawn((main_menu_style(), ScoreboardScreen))
        .with_children(|parent| {
            // Title
            parent.spawn((Text::new("High scores"), text_style(48.0, &asset_server)));

            if high_scores.scores.is_empty() {
                parent.spawn((Text::new("No scores yet"), text_style(28.0, &asset_server)));
            } else {
                for row in rows.iter() {
                    parent
                        .spawn(Node {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        })
                        .with_children(|row_node| {
                            for (cell, width) in row.iter().zip(COLUMN_WIDTHS) {
                                row_node.spawn((
                                    Text::new(cell.clone()),
                                    text_style(24.0, &asset_server),
                                    Node {
                                        width: Val::Px(width),
                                        ..default()
                                    },
                                ));
                            }
                        });
                }
            }

            // Back
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Interaction::None,
                    BackButton,
                ))
                .with_children(|button| {
                    button.spawn((Text::new("Back"), text_style(32.0, &asset_server)));
                });
        });
}

pub fn despawn_scoreboard(
    mut commands: Commands,
    screen_query: Query<Entity, With<ScoreboardScreen>>,
) {
    if let Ok(entity) = screen_query.single() {
        commands.entity(entity).despawn();
    }
}

pub fn interact_with_back_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BackButton>),
    >,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_state.set(AppState::MainMenu);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}
