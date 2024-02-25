use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;

use crate::components::collider::Collider;
use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct Pipe;

const X_SPAWN: f32 = 10.0;
const PIPE_HEIGHT: f32 = 10.0;

/// Spawns a pipe (either coming from the top or the bottom of the screen) with the end at the specified height
///
/// # Arguments
///
/// * `is_top` - if the pipe is coming from the top or the bottom of the screen
/// * `outlet_y` - the height at which the pipe ands
/// * `pipe_width` - the width of the pipe
///
/// # Examples
///
/// ```
/// use bevy_birb::entities::pipe::spawn_pipe;
///
/// spawn_pipe(commands, pipe_width, outlet_y, is_top);
/// ```
pub fn spawn_pipe(mut commands: Commands, pipe_width: f32, outlet_y: f32, is_top: bool) {
    let mut spawn_height: f32 = 0.0;
    if is_top {
        spawn_height = outlet_y + (PIPE_HEIGHT / 2.0)
    } else {
        spawn_height = outlet_y - (PIPE_HEIGHT / 2.0)
    }

    commands.spawn((
        Pipe,
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(pipe_width, PIPE_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, spawn_height, 0.0),
            ..Default::default()
        },
        Collider::Rect {
            width: pipe_width,
            height: PIPE_HEIGHT,
        },
        Velocity { x: -1.5, y: 0.0 },
    ));
}
