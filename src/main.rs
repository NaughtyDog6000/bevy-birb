use crate::systems::display_fps_system::{fps_text_update_system, setup_fps_counter};
use crate::systems::{gravity_system::apply_gravity, velocity_system::apply_velocity};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::WindowMode;
use bevy::{prelude::*, render::camera::ScalingMode};
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
                name: Some(String::from("Flappy Birb")),
                mode: bevy::window::WindowMode::Windowed,
                prevent_default_event_handling: true,
                transparent: true,
                resizable: true,

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
                toggle_fullscreen_system,
                systems::pipes_system::spawn_pipes,
                // log_position,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: 16.0,
        height: 9.0,
    };

    commands.spawn(camera_bundle);
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(-1.0, -1.0, 0.0),
        ..Default::default()
    });

    // start playing background music immediately
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/music/Monkeys-Spinning-Monkeys.ogg"),
        ..Default::default()
    });
}

fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    _time: Res<Time>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowUp])
        | touch_inputs.any_just_pressed()
    {
        println!("pressed");

        for mut velocity in &mut players {
            velocity.y = 7.0;
        }
    }
}

fn toggle_fullscreen_system(
    _commands: Commands,
    mut windows: Query<&mut Window>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if input.just_pressed(KeyCode::F11) {
        if window.mode != WindowMode::BorderlessFullscreen {
            window.mode = WindowMode::BorderlessFullscreen;
        } else {
            window.mode = WindowMode::Windowed;
        }
    }
}

#[allow(dead_code)]
fn log_position(
    camera_query: Query<(&Transform, &OrthographicProjection), With<Camera>>,
    windows: Query<&Window>,
) {
    let (transform, projection) = camera_query.single();

    let Some(_cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let cam_center = transform.translation;
    let cam_width = projection.area.width();
    let cam_height = projection.area.height();

    println!(
        "center: {}, width: {}, height: {}",
        cam_center, cam_width, cam_height
    );
}
