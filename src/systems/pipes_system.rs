use crate::entities::pipe::{spawn_double_pipe, spawn_pipe, Pipe};
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::score_system::GameScore;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

const DESPAWN_PAST_X: f32 = -10.0;

pub fn spawn_pipes(
    time: Res<Time>,
    mut game_score: ResMut<GameScore>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    pipes_query: Query<(Entity, &Transform), With<Pipe>>,
) {
    // check if any pipes are far enough passed the screen to despawn them so that we arent continuously increasing the number of entities in the world
    for (entity, transform) in pipes_query.iter() {
        if transform.translation.x <= DESPAWN_PAST_X {
            commands.entity(entity).despawn();
        }
    }

    // if sufficient time since the last spawn has passed, spawn another pipe
    if spawn_timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();

        let gap_y = rng.gen_range(-5..5) as f32;

        let hole_size: f32 = 4.0 - (game_score.game_time.elapsed_secs() * 0.2);
        spawn_double_pipe(&mut commands, 1.0, gap_y, hole_size);
    }
}
