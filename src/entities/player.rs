use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

use crate::components::gravity::Gravity;
use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        
        Velocity { x: 0.0, y: 0.0 },
        Gravity { x: 0.0, y: -10.0 },
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(1.0, 1.0)),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}
