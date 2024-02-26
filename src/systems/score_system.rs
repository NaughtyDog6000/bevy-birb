use bevy::prelude::*;

#[derive(Resource)]
pub struct GameScore {
    pub score: f32,
    pub game_start: f32,
    pub game_end: f32,
}

impl Default for GameScore {
    fn default() -> Self {
        Self {
            score: Default::default(),
            game_start: Default::default(),
            game_end: Default::default(),
        }
    }
}

pub fn update_score(time: Res<Time>, mut game_score: ResMut<GameScore>) {
    game_score.score += time.delta_seconds();
    // println!("{}", game_score.score);
}
