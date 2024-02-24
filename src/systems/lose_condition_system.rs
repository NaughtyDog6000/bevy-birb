// checks if the lose state conditions for the game are met

// check for player collision events with pipe
// check for player position out of bounds of screen

// create player loss event
use bevy::prelude::*;
use bevy::{
    ecs::{event::EventReader, system::Query},
    render::camera::OrthographicProjection,
    transform::components::{GlobalTransform, Transform},
};

use crate::entities::player::Player;

use super::collision_system::PlayerCollisionEvent;

pub fn check_for_lose_conditions(
    mut _player_collision_event: EventReader<PlayerCollisionEvent>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
) {
    let camera = camera_query.single();
    let player_transform: &Transform = player_query.single();

    let cam_center = camera.0.compute_transform().translation.xy();
    let cam_width = camera.1.area.width();
    let cam_height = camera.1.area.height();

    // if the player is below the screen
    println!("player height: {}", player_transform.translation.y);
}
