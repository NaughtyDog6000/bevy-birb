use crate::game_over_input::game_over_input;
use crate::systems::display_fps_system::{fps_text_update_system, setup_fps_counter};
use crate::systems::{gravity_system::apply_gravity, velocity_system::apply_velocity};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::window::WindowMode;
use bevy::winit::WinitSettings;
use bevy::{prelude::*, render::camera::ScalingMode};
use components::velocity::Velocity;
use entities::player::Player;
use systems::pipes_system::SpawnTimer;
use systems::player_actions::flap::player_flap;
use systems::player_actions::{self, game_over_input};

pub mod components;
pub mod entities;
pub mod state;
pub mod systems;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
pub enum GameState {
    #[default]
    InGame,
    GameOver,
    Menu,
    Settings,
}

#[derive(Component)]
pub struct MusicMarker;

#[derive(Component)]
pub struct DontDespawnOnRestart;

fn main() {
    App::new()
        .insert_resource(SpawnTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
        .insert_resource(WinitSettings::game())
        .init_state::<GameState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some(String::from("Flappy Birb")),
                mode: bevy::window::WindowMode::Windowed,
                prevent_default_event_handling: true,
                // canvas: Some("#birb-canvas".into()),
                transparent: true,
                resizable: true,

                ..default()
            }),
            ..default()
        }))
        .add_event::<systems::collision_system::PlayerCollisionEvent>()
        .add_plugins((FrameTimeDiagnosticsPlugin::default(),))
        // systems that should run all the time regardless of state
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                fps_text_update_system,
                player_actions::toggle_fullscreen::toggle_fullscreen_system,
            ),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                reset_game,
                entities::player::spawn_player,
                setup_game_start,
                setup_fps_counter,
            )
                .chain(),
        )
        .add_systems(Startup, application_setup)
        // systems that run when the game is such as movement and flap input
        .add_systems(
            Update,
            (
                player_flap,
                (apply_gravity, apply_velocity).chain(),
                systems::pipes_system::spawn_pipes,
                systems::collision_system::check_for_collisions_with_player,
                systems::lose_condition_system::check_for_lose_conditions,
                player_actions::audio_playback::toggle_music_playback,
                // log_position,
            )
                .run_if(in_state(GameState::InGame)),
        )
        // systems that run on game over state such as gameover_input
        .add_systems(
            Update,
            (game_over_input).run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup_game_start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(-1.0, -1.0, 0.0),
        ..Default::default()
    });
}

fn application_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::Fixed {
        width: 16.0,
        height: 9.0,
    };

    commands.spawn(camera_bundle);

    // start playing background music immediately
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/music/Monkeys-Spinning-Monkeys.ogg"),
            settings: PlaybackSettings {
                paused: true,
                ..Default::default()
            },
            ..Default::default()
        },
        MusicMarker,
        DontDespawnOnRestart,
    ));
}

fn reset_game(
    mut commands: Commands,
    entites: Query<
        Entity,
        (
            Without<Camera>,
            Without<Window>,
            Without<DontDespawnOnRestart>,
        ),
    >,
) {
    println!("reseting the game");
    for entity in &entites {
        commands.entity(entity).despawn();
    }
}

#[allow(dead_code)]
fn log_position(
    camera_query: Query<(&GlobalTransform, &OrthographicProjection), With<Camera>>,
    windows: Query<&Window>,
) {
    let (transform, projection) = camera_query.single();

    let Some(_cursor_position) = windows.single().cursor_position() else {
        return;
    };
    let cam_center = transform.compute_transform().translation.xy();
    let cam_width = projection.area.width();
    let cam_height = projection.area.height();

    println!(
        "center: {}, width: {}, height: {}",
        cam_center, cam_width, cam_height
    );
}
