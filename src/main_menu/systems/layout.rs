use bevy::prelude::*;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_main_menu(mut commands: Commands<'_, '_>, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
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
        .spawn((main_menu_style(), MainMenu {}))
        .with_children(|parent| {
            // Title
            parent
                .spawn((title_button_style(),)) // <-- spawn closed here
                .with_children(|title| {
                    // <-- .with_children chained on it
                    // Image 1
                    title.spawn((
                        image_style(),
                        ImageNode::new(asset_server.load("sprites/ball_blue_large.png")),
                    ));

                    // Text
                    title.spawn((Text::new("Ball Game"), text_style(64.0, &asset_server)));

                    // Image 2
                    title.spawn((
                        image_style(),
                        ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                    ));
                });
            // Play
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Interaction::None,
                    PlayButton {},
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Play"), // text content
                        text_style(32.0, &asset_server),
                    ));
                });
            // Quit
            parent
                .spawn((
                    button_style(),
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                    Interaction::None,
                    QuitButton {},
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Quit"), // text content
                        text_style(32.0, &asset_server),
                    ));
                });
        })
        .id();

    return main_menu_entity;
}
