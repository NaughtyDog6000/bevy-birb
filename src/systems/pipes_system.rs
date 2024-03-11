use crate::entities::{pipe::{self, spawn_double_pipe, Pipe}, player::Player};
use bevy::ecs::system::Commands;
use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::score_system::{AddScoreEvent, GameScore};

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

const DESPAWN_PAST_X: f32 = -10.0;

pub fn spawn_pipes(
    time: Res<Time>,
    game_score: ResMut<GameScore>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    mut pipes_query: Query<(Entity, &Transform, &mut Pipe), With<Pipe>>,
    player_query: Query<&Transform, With<Player>>,
    mut score_add_event: EventWriter<AddScoreEvent>,

) {
    let player_pos = player_query.single();
    // check if any pipes are far enough passed the screen to despawn them so that we arent continuously increasing the number of entities in the world
    for (entity, transform, mut pipe) in pipes_query.iter_mut() {
        // if the pipe isnt marked as passed the player yet, check if its further left than the player
        if !pipe.passed_player {
            if transform.translation.x <= player_pos.translation.x {
                // add score 
                // TODO due to there being two pipes (one above and one bellow) this ususally fires twice
                score_add_event.send(AddScoreEvent(100));
                // mark as passed
                pipe.passed_player = true;

            }
        }


        if transform.translation.x <= DESPAWN_PAST_X {
            commands.entity(entity).despawn();
        }
    }

    // if sufficient time since the last spawn has passed, spawn another pipe
    if spawn_timer.0.tick(time.delta()).just_finished() {
        let mut rng = thread_rng();

        let gap_y = rng.gen_range(-3.5..3.5) as f32;

        let hole_size: f32 = 4.0 - f32::min(game_score.game_time.elapsed_secs() * 0.02, 3.0);
        spawn_double_pipe(&mut commands, 1.0, gap_y, hole_size);
    }
}
