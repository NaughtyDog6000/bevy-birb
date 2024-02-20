use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

use crate::components::gravity::Gravity;
use crate::components::velocity::Velocity;

pub fn apply_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
