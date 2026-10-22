use bevy::input::ButtonState;
use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::prelude::*;

use super::components::*;
use crate::AppState;
use crate::game::score::resources::PlayerName;
use crate::main_menu::styles::*;

pub fn spawn_name_entry(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((main_menu_style(), NameEntryScreen))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Enter your name"),
                text_style(48.0, &asset_server),
            ));

            // Input box
            parent
                .spawn((
                    Node {
                        width: Val::Px(400.0),
                        height: Val::Px(60.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(2.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                    BorderColor::all(Color::WHITE),
                ))
                .with_children(|input| {
                    input.spawn((
                        Text::new("_"),
                        text_style(32.0, &asset_server),
                        NameInputText,
                    ));
                });

            // Hint
            parent.spawn((
                Text::new("Type your name and press Enter"),
                text_style(24.0, &asset_server),
            ));

            // Start
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Interaction::None,
                    StartButton,
                ))
                .with_children(|button| {
                    button.spawn((Text::new("Start"), text_style(32.0, &asset_server)));
                });

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

pub fn despawn_name_entry(
    mut commands: Commands,
    screen_query: Query<Entity, With<NameEntryScreen>>,
) {
    if let Ok(entity) = screen_query.single() {
        commands.entity(entity).despawn();
    }
}

/// Start the game when the name is not empty. The name is stored without surrounding spaces.
fn start_game_if_valid(player_name: &mut PlayerName, next_state: &mut NextState<AppState>) {
    let trimmed = player_name.0.trim().to_string();
    if trimmed.is_empty() {
        return;
    }
    player_name.0 = trimmed;
    next_state.set(AppState::Game);
}

/// Text input: letters, digits, '-', '_' and spaces; Backspace removes, Enter starts the game.
pub fn name_typing(
    mut keyboard_events: MessageReader<KeyboardInput>,
    mut player_name: ResMut<PlayerName>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for event in keyboard_events.read() {
        if event.state != ButtonState::Pressed {
            continue;
        }

        match &event.logical_key {
            Key::Enter => start_game_if_valid(&mut player_name, &mut next_state),
            Key::Backspace => {
                player_name.0.pop();
            }
            Key::Space => {
                if !player_name.0.is_empty() && player_name.0.chars().count() < PlayerName::MAX_LEN
                {
                    player_name.0.push(' ');
                }
            }
            Key::Character(characters) => {
                for c in characters.chars() {
                    if (c.is_alphanumeric() || c == '-' || c == '_')
                        && player_name.0.chars().count() < PlayerName::MAX_LEN
                    {
                        player_name.0.push(c);
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn update_name_text(
    player_name: Res<PlayerName>,
    mut query: Query<&mut Text, With<NameInputText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("{}_", player_name.0));
    }
}

pub fn interact_with_start_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut player_name: ResMut<PlayerName>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                start_game_if_valid(&mut player_name, &mut next_state);
            }
            Interaction::Hovered => *background_color = HOVERED_BUTTON_COLOR.into(),
            Interaction::None => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
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
