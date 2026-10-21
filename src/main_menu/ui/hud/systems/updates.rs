use bevy::prelude::*;

use crate::game::enemy::components::Enemy;
use crate::game::score::resources::Score; // adjust if Score lives elsewhere
use crate::main_menu::ui::hud::components::*;

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        debug!("Update score stars: {}", score.value);
        if let Ok(mut text) = query.single_mut() {
            *text = Text::new(score.value.to_string());
        }
    }
}

pub fn update_enemy_text(
    enemies: Query<(), With<Enemy>>, // count live enemies
    mut query: Query<&mut Text, With<EnemyText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(enemies.iter().count().to_string());
    }
}
