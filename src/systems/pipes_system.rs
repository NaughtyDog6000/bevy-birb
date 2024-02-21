

use crate::entities::pipe::spawn_pipe;
use bevy::ecs::{system::Commands};
use bevy::prelude::*;

#[derive(Resource)]
pub struct SpawnTimer(pub Timer);

pub fn spawn_pipes(time: Res<Time>, mut spawn_timer: ResMut<SpawnTimer>, commands: Commands) {
    if spawn_timer.0.tick(time.delta()).just_finished() {
        spawn_pipe(commands, 2.0, 2.0);
    }
}
