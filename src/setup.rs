use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

use crate::{DontDespawnOnRestart, MusicMarker};

pub fn application_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    // TODO re-write as a vec of tupples containing key and action values which is turned into a string
    let text: String = "
    [Space]: Flap,
    [Escape]: Exit Game,
    [M]: Toggle Mute Audio,
    [R]: Restart when dead,
    [F11]: Toggle Fullscreen,
    "
    .to_string();

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Keybinds: ",
                TextStyle {
                    font_size: 30.0,
                    ..Default::default()
                },
            ),
            TextSection::new(
                text,
                TextStyle {
                    font_size: 20.0,
                    ..Default::default()
                },
            ),
        ])
        .with_style(Style {
            max_width: Val::Percent(90.0),
            ..Default::default()
        }),
        DontDespawnOnRestart,
    ));
}
