use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;
use crate::components::gravity::Gravity;
use crate::components::velocity::Velocity;

#[derive(Resource)]
pub struct SpawnTimer(Timer);

pub fn spawn_pipes(time: Res<Time>, mut spawn_timer: ResMut<SpawnTimer>, mut commands: Commands) {
    if timer.0.tick(time.delta()).just_finished() {
        spawn_pipe(
            commands,
            2.0,
            2.0,
        );
    }

}