use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

use crate::components::gravity::Gravity;
use crate::components::velocity::Velocity;

pub fn apply_gravity(time: Res<Time>, mut query: Query<(&Gravity, &mut Velocity)>) {
    for (gravity, mut velocity) in query.iter_mut() {
        velocity.x += gravity.x * time.delta_seconds();
        velocity.y += gravity.y * time.delta_seconds();
    }
}
