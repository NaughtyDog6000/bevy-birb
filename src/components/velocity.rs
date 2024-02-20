use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
