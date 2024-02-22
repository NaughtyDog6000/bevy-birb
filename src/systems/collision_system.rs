use bevy::prelude::*;

pub fn check_for_collisions(
    mut commands: Commands,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>
) {

    // add to collision Events 
}