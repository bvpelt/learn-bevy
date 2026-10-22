use bevy::prelude::*;

use super::resources::*;
use crate::events::GameOver;

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default()); // insert score with initial value 0
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        debug!("Score updated: {}", score.value.to_string());
    }
}

pub fn update_high_scores(
    mut game_over_event_reader: MessageReader<GameOver>,
    player_name: Res<PlayerName>,
    mut high_scores: ResMut<HighScores>,
) {
    for event in game_over_event_reader.read() {
        let entry = HighScoreEntry {
            player_name: player_name.0.clone(),
            level: event.level,
            score: event.score,
        };

        if high_scores.submit(entry) {
            high_scores.save();
        }
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        debug!("High Scores: {:?}", high_scores)
    }
}
