use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

#[derive(Component)]
pub struct Gravity {
    pub x: f32,
    pub y: f32,
}
