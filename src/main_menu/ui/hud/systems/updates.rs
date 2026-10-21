use bevy::prelude::*;

use crate::game::score::resources::{EnemiesHit, Score}; // adjust if Score lives elsewhere
use crate::main_menu::ui::hud::components::*;

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        println!("Update score stars: {}", score.value);
        if let Ok(mut text) = query.single_mut() {
            *text = Text::new(score.value.to_string());
        }
    }
}

pub fn update_enemy_text(
    // adapt to whatever resource tracks hits — example uses an EnemiesHit resource
    enemies_hit: Res<EnemiesHit>,
    mut query: Query<&mut Text, With<EnemyText>>,
) {
    if enemies_hit.is_changed() {
        println!("Enemie hits: {}", enemies_hit.value);
        if let Ok(mut text) = query.single_mut() {
            *text = Text::new(enemies_hit.value.to_string());
        }
    }
}
