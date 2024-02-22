use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::collider::Collider;
use crate::components::gravity::Gravity;
use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Collider::Circle { radius: 0.5 },
        Velocity { x: 0.0, y: 0.0 },
        Gravity { x: 0.0, y: -0.0 },
        MaterialMesh2dBundle {
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            mesh: Mesh2dHandle(meshes.add(Circle { radius: 0.5 })),
            ..Default::default()
        },
    ));
}
