use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct Pipe;

pub fn spawn_pipe(mut commands: Commands) {
    commands.spawn((
        Velocity { x: -1.0, y: 0.0 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(1.5, 4.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
