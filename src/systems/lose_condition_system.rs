// checks if the lose state conditions for the game are met

// check for player collision events with pipe
// check for player position out of bounds of screen

// create player loss event

use bevy::ecs::event::EventReader;

pub fn check_for_lose_conditions(
    mut player_collision_event: EventReader<super::collision_system::PlayerCollisionEvent>,
) {
}
