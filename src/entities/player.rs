use bevy::ecs::{component::Component, system::Commands};
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::components::collider::Collider;
use crate::components::gravity::Gravity;
use crate::components::velocity::Velocity;

#[derive(Component)]
pub struct Player;

const PLAYER_RADIUS: f32 = 0.5;
const PLAYER_GRAVITY: f32 = -10.0;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Collider::Circle {
            radius: PLAYER_RADIUS,
        },
        Velocity { x: 0.0, y: 0.0 },
        Gravity {
            x: 0.0,
            y: PLAYER_GRAVITY,
        },
        MaterialMesh2dBundle {
            transform: Transform {
                translation: Vec3::new(-5.0, 0.0, 0.0),
                ..Default::default()
            },
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: PLAYER_RADIUS,
            })),
            ..Default::default()
        },
    ));
}
