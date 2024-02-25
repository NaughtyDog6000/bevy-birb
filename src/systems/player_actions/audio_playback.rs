use bevy::prelude::*;

use crate::MusicMarker;

pub fn toggle_music_playback(
    input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MusicMarker>>,
) {
    if input.any_just_pressed([KeyCode::MediaPlayPause, KeyCode::KeyM]) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}
