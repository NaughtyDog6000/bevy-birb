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

use crate::entities::pipe::Pipe;
use crate::entities::player::Player;
use crate::GameState;

use super::collision_system::PlayerCollisionEvent;

pub fn check_for_lose_conditions(
    // mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut player_collision_event: EventReader<PlayerCollisionEvent>,
    pipes_query: Query<&Pipe>,
    player_query: Query<&Transform, With<Player>>,
    camera_query: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
) {
    let camera = camera_query.single();
    let player_transform: &Transform = player_query.single();

    // let cam_center = camera.0.compute_transform().translation.xy();
    // let cam_width = camera.1.area.width();
    let cam_height = camera.1.area.height();

    // println!("player height: {}", player_transform.translation.y);

    // if the player is below the screen
    if player_transform.translation.y < -cam_height / 2.0 {
        println!("player bellow screen");
        next_state.set(GameState::GameOver);
    }

    // if the player is above the screen
    if player_transform.translation.y > cam_height / 2.0 {
        println!("player above screen");
        next_state.set(GameState::GameOver);
    }

    // if the player has collided with a pipe
    for collision_event in player_collision_event.read() {
        if let Ok(_) = pipes_query.get(collision_event.0) {
            next_state.set(GameState::GameOver);
        };
    }
}
