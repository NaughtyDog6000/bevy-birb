use std::time::Instant;

use bevy::{prelude::*, time::Stopwatch};

use crate::setup::ScoreMarker;

#[derive(Resource)]
pub struct GameScore {
    pub score: u32,
    pub game_start: Stopwatch,
    pub game_end: f32,
}

pub struct AddScore(u32);

impl Default for GameScore {
    fn default() -> Self {
        Self {
            score: Default::default(),
            game_start: Stopwatch::new(),
            game_end: Default::default(),
        }
    }
}

pub fn update_score(
    time: Res<Time>,
    mut game_score: ResMut<GameScore>,
    mut score_text_query: Query<&mut Text, With<ScoreMarker>>,
) {
    // Display the points that will be added for time alive, should the game end now, but dont add to the score
    // add 1 point for every milisecond alive

    // game_score.score += (time.delta_seconds() * 1000.0) as u32;
    game_score.game_start.tick(time.delta());

    // println!("{}", game_score.score);
    for mut text in &mut score_text_query {
        text.sections[1].value = format!("{0:.2}", game_score.game_start.elapsed_secs() * 2.0);
    }
}
