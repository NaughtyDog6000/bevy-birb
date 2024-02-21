use crate::systems::display_fps_system::{fps_text_update_system, setup_fps_counter};
use crate::systems::{gravity_system::apply_gravity, velocity_system::apply_velocity};
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{
    input::{
        keyboard::{Key, KeyboardInput},
        InputPlugin,
    },
    prelude::*,
    render::camera::ScalingMode,
};
use components::velocity::Velocity;
use entities::player::Player;
use systems::pipes_system::SpawnTimer;

pub mod components;
pub mod entities;
pub mod systems;

fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                prevent_default_event_handling: false,
                mode: bevy::window::WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((FrameTimeDiagnosticsPlugin::default(),))
        .add_systems(
            Startup,
            (setup, setup_fps_counter, entities::player::spawn_player),
        )
        .add_systems(
            Update,
            (
                move_player,
                bevy::window::close_on_esc,
                (apply_gravity, apply_velocity).chain(),
                fps_text_update_system,
            ),
        )
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.0);
    commands.spawn(camera_bundle);
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowUp]) {
        println!("pressed");

        for mut velocity in &mut players {
            velocity.y = 7.0;
        }
    }
}
