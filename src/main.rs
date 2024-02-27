use crate::game_over_input::game_over_input;
use crate::systems::display_fps_system::{fps_text_update_system, setup_fps_counter};
use crate::systems::{gravity_system::apply_gravity, velocity_system::apply_velocity};
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;

use bevy::prelude::*;
use bevy::winit::WinitSettings;

use setup::application_setup;

use state::on_enter_playing::on_enter_play_state;
use systems::pipes_system::SpawnTimer;
use systems::player_actions::flap::player_flap;
use systems::player_actions::{self, game_over_input};
use systems::score_system::GameScore;

pub mod components;
pub mod entities;
pub mod setup;
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
        .insert_resource(GameScore::default())
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
        .add_event::<systems::score_system::AddScoreEvent>()
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_systems(Startup, application_setup)
        // systems that should run all the time regardless of state
        .add_systems(
            Update,
            (
                bevy::window::close_on_esc,
                fps_text_update_system,
                player_actions::toggle_fullscreen::toggle_fullscreen_system,
            ),
        )
        // Systems that run when changing into the ingame state
        .add_systems(
            OnEnter(GameState::InGame),
            (
                reset_game,
                on_enter_play_state,
                entities::player::spawn_player,
                setup_fps_counter,
            )
                .chain(),
        )
        // systems that run when the in the ingame state
        .add_systems(
            Update,
            (
                player_flap,
                (apply_gravity, apply_velocity).chain(),
                systems::pipes_system::spawn_pipes,
                systems::collision_system::check_for_collisions_with_player,
                systems::lose_condition_system::check_for_lose_conditions,
                systems::score_system::update_score,
                player_actions::audio_playback::toggle_music_playback,
                // log_position,
            )
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            OnEnter(GameState::GameOver),
            (state::on_enter_game_over::on_enter_gameover_state,).chain(),
        )
        // systems that run on game over state such as gameover_input
        .add_systems(
            Update,
            (game_over_input).run_if(in_state(GameState::GameOver)),
        )
        .run();
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
    mut game_score: ResMut<GameScore>,
) {
    println!("current score: {}", game_score.score);
    *game_score = GameScore::default();

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

// #[allow(dead_code)]
// fn test_rand() {
//     let mut rng = thread_rng();
// }
