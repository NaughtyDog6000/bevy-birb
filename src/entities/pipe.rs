use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

use crate::components::collider::Collider;
use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct Pipe;

pub fn spawn_pipe(mut commands: Commands, _position_x: f32, _position_y: f32) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            Pipe,
            Velocity { x: -0.5, y: 0.0 },
        ))
        .with_children(|parent| {
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        custom_size: Some(Vec2::new(0.5, 1.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, 1.0, 0.0),
                    ..Default::default()
                },
                Collider::Rect {
                    width: 0.5,
                    height: 1.0,
                },
            ));
            parent.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.0, 1.0),
                        custom_size: Some(Vec2::new(0.5, 1.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(0.0, -1.0, 0.0),
                    ..Default::default()
                },
                Collider::Rect {
                    width: 0.5,
                    height: 1.0,
                },
            ));
        });
}
