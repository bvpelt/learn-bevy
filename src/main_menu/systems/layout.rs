use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands<'_, '_>, asset_server: Res<AssetServer>) {
    let main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn despawn_main_menu(
    mut commands: Commands<'_, '_>,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.single() {
        commands.entity(main_menu_entity).despawn();
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                row_gap: Val::Px(8.0), // or UiValue::Px(8.0) for one axis
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // or Color::RED constant
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
            parent
                .spawn((title_button_style(),)) // <-- spawn closed here
                .with_children(|title| {
                    // <-- .with_children chained on it
                    // Image 1
                    title.spawn((
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
                        },
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                    ));

                    // Text
                    title.spawn((
                        Text::new("Ball Game"),
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(),
                            font_size: FontSize::Px(64.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    // Image 2
                    title.spawn((
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
                        },
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                    ));
                });
            // Play
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    PlayButton {},
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Play"), // text content
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(), // Handle -> FontSource
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        }, // centers the text in its node
                    ));
                });
            // Quit
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    QuitButton {},
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Quit"), // text content
                        TextFont {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf").into(), // Handle -> FontSource
                            font_size: FontSize::Px(32.0),
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        TextLayout {
                            justify: Justify::Center,
                            ..default()
                        }, // centers the text in its node
                    ));
                });
        })
        .id();

    return main_menu_entity;
}
