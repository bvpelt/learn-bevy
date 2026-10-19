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
                column_gap: Val::Px(8.0), // or UiValue::Px(8.0) for one axis
                ..default()
            },
            BackgroundColor(Color::srgb(0.8, 0.2, 0.2)), // or Color::RED constant
            MainMenu {},
        ))
        .with_children(|parent| {
            // Title
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
        })
        .id();

    return main_menu_entity;
}
