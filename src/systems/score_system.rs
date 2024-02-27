use bevy::utils::Instant;
use bevy::{prelude::*, time::Stopwatch};

use crate::setup::ScoreMarker;

#[derive(Resource)]
pub struct GameScore {
    pub score: u32,
    pub game_start: Instant,
    pub game_end: Instant,
    pub game_time: Stopwatch,
}
#[derive(Event)]
pub struct AddScoreEvent(pub u32);

impl Default for GameScore {
    fn default() -> Self {
        Self {
            score: Default::default(),
            game_start: Instant::now(),
            game_end: Instant::now(),
            game_time: Stopwatch::new(),
        }
    }
}

pub fn update_score(
    time: Res<Time>,
    mut game_score: ResMut<GameScore>,
    mut score_text_query: Query<&mut Text, With<ScoreMarker>>,
    mut score_add_event: EventReader<AddScoreEvent>,
) {
    for score in score_add_event.read() {
        game_score.score += score.0;
    }

    // Display the points that will be added for time alive, should the game end now, but dont add to the score
    // add 1 point for every milisecond alive

    game_score.game_time.tick(time.delta());

    // println!("{}", game_score.score);
    for mut text in &mut score_text_query {
        text.sections[1].value = format!(
            "{0:.2}",
            (game_score.score + (game_score.game_time.elapsed_secs()) as u32)
        );
    }
}
