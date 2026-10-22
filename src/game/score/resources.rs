use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::ErrorKind;

/// File in the working directory (the project root when using `cargo run`).
pub const HIGH_SCORES_FILE: &str = "highscores.json";
pub const MAX_HIGH_SCORES: usize = 10;

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

// default trait implementation for Score
impl Default for Score {
    fn default() -> Self {
        Score { value: 0 }
    }
}

/// Name of the current player. Survives between games so it can be prefilled.
#[derive(Resource, Debug, Default)]
pub struct PlayerName(pub String);

impl PlayerName {
    pub const MAX_LEN: usize = 12;
}

/// One line of the scoreboard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HighScoreEntry {
    pub player_name: String,
    pub level: u32,
    pub score: u32,
}

/// The best score per player, highest first, at most MAX_HIGH_SCORES entries.
#[derive(Resource, Debug, Default)]
pub struct HighScores {
    pub scores: Vec<HighScoreEntry>,
}

impl HighScores {
    /// Read the scores from the json file. A missing or broken file gives an empty list.
    pub fn load() -> Self {
        let mut high_scores = match fs::read_to_string(HIGH_SCORES_FILE) {
            Ok(content) => match serde_json::from_str::<Vec<HighScoreEntry>>(&content) {
                Ok(scores) => HighScores { scores },
                Err(error) => {
                    warn!("Could not parse {}: {}", HIGH_SCORES_FILE, error);
                    HighScores::default()
                }
            },
            Err(error) if error.kind() == ErrorKind::NotFound => HighScores::default(),
            Err(error) => {
                warn!("Could not read {}: {}", HIGH_SCORES_FILE, error);
                HighScores::default()
            }
        };
        high_scores.sort_and_truncate();
        high_scores
    }

    /// Write the scores to the json file.
    pub fn save(&self) {
        match serde_json::to_string_pretty(&self.scores) {
            Ok(json) => {
                if let Err(error) = fs::write(HIGH_SCORES_FILE, json) {
                    error!("Could not write {}: {}", HIGH_SCORES_FILE, error);
                }
            }
            Err(error) => error!("Could not serialize high scores: {}", error),
        }
    }

    /// Offer the result of a game. A player only has one entry: the best score.
    /// Returns true when the list has changed and should be saved.
    pub fn submit(&mut self, entry: HighScoreEntry) -> bool {
        let existing = self
            .scores
            .iter()
            .position(|e| e.player_name.to_lowercase() == entry.player_name.to_lowercase());

        match existing {
            Some(index) => {
                if entry.score > self.scores[index].score {
                    self.scores[index] = entry;
                } else {
                    return false;
                }
            }
            None => self.scores.push(entry),
        }

        self.sort_and_truncate();
        true
    }

    fn sort_and_truncate(&mut self) {
        // highest score first, on equal score the higher level first
        self.scores
            .sort_by(|a, b| b.score.cmp(&a.score).then(b.level.cmp(&a.level)));
        self.scores.truncate(MAX_HIGH_SCORES);
    }
}
