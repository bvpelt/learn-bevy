use bevy::prelude::*;

use crate::main_menu::ui::hud::components::*;
use crate::main_menu::ui::hud::styles::*;

pub fn spawn_hud_menu(mut commands: Commands<'_, '_>, asset_server: Res<AssetServer>) {
    let _hud_menu_entity = build_hud_menu(&mut commands, &asset_server);
}

pub fn despawn_hud_menu(
    mut commands: Commands<'_, '_>,
    hud_menu_query: Query<Entity, With<HudMenu>>,
) {
    if let Ok(hud_menu_entity) = hud_menu_query.single() {
        commands.entity(hud_menu_entity).despawn();
    }
}

pub fn build_hud_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    commands
        .spawn((
            hud_menu_style(),
            BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
            HudMenu {},
        ))
        .with_children(|hud| {
            // Left group: [star image] [score]
            hud.spawn(hud_group_style()).with_children(|group| {
                group.spawn((
                    hud_icon_style(),
                    ImageNode::new(asset_server.load("sprites/star.png")),
                ));
                group.spawn((hud_text_style(asset_server), ScoreText));
            });

            // Right group: [enemy count] [ball image]
            hud.spawn(hud_group_style()).with_children(|group| {
                group.spawn((hud_text_style(asset_server), EnemyText));
                group.spawn((
                    hud_icon_style(),
                    ImageNode::new(asset_server.load("sprites/ball_red_large.png")),
                ));
            });
        })
        .id()
}
