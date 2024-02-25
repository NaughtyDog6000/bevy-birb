use crate::entities::pipe::{spawn_pipe, Pipe};
use bevy::ecs::system::Commands;
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn spawn_pipes(
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    pipes_query: Query<(Entity, &Transform), With<Pipe>>,
) {
    // check if any pipes are far enough passed the sc
    for (entity, transform) in pipes_query.iter() {
        if transform.translation.x <= -1.0 {
            commands.entity(entity).despawn();
        }
    }

    // if sufficient time since the last spawn has passed, spawn another pipe
    if spawn_timer.0.tick(time.delta()).just_finished() {
        spawn_pipe(commands, 2.0, -3.0, false);
    }
}
