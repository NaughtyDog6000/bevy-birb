use bevy::{
    math::bounding::{Aabb2d, BoundingCircle, IntersectsVolume},
    prelude::*,
};

use crate::{components::collider::Collider, entities::player::Player};

pub fn check_for_collisions_with_player(
    mut commands: Commands,
    mut query: Query<(&mut GlobalTransform, &Collider), Without<Player>>,
    mut player_query: Query<(&Transform, &Collider), With<Player>>,
) {
    let (player_transform, player_collider) = player_query.single_mut();
    let bounds = match player_collider {
        Collider::Circle { radius } => {
            BoundingCircle::new(player_transform.translation.truncate(), *radius)
        }
        Collider::Rect { width, height: _ } => {
            BoundingCircle::new(player_transform.translation.truncate(), width / 2.0)
        }
    };

    for (transform, collider) in query.iter() {
        let overlap = match collider {
            Collider::Circle { radius } => bounds.intersects(&BoundingCircle::new(
                transform.compute_transform().translation.xy(),
                *radius,
            )),
            Collider::Rect { width, height } => bounds.intersects(&Aabb2d::new(
                transform.compute_transform().translation.xy(),
                Vec2 {
                    x: *width / 2.0,
                    y: *height / 2.0,
                },
            )),
        };
        if overlap {
            println!(
                "overlap with obj of transform: {}",
                transform.compute_transform().translation.xy()
            );
        }
    }
    // add to collision Events
}
