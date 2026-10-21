use bevy::prelude::*;

use crate::game::score::resources::Score;
use crate::main_menu::ui::game_over_menu::components::FinalScoreText;

pub fn update_final_score_text(
    score: Res<Score>,
    mut query: Query<&mut Text, With<FinalScoreText>>,
) {
    if let Ok(mut text) = query.single_mut() {
        *text = Text::new(format!("Your final score was: {}", score.value));
    }
}
