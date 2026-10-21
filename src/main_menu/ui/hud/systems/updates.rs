use bevy::prelude::*;

use crate::game::score::resources::Score;
use crate::ui::hud::components::*;

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            *text = Text::new(score.value.to_string());
        }
    }
}

pub fn update_enemy_text(
    // adapt to whatever resource tracks hits — example uses an EnemiesHit resource
    enemies: Res<crate::game::enemy::resources::EnemiesHit>,
    mut query: Query<&mut Text, With<EnemyText>>,
) {
    if enemies.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            *text = Text::new(enemies.value.to_string());
        }
    }
}
