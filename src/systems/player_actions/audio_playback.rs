use bevy::prelude::*;

use crate::{setup::MusicMarker, systems::player_actions::actions::PlayerAction, InputBindings};

use super::actions::ActionEvent;

pub fn toggle_music_playback(
    _input: Res<ButtonInput<KeyCode>>,
    music_controller: Query<&AudioSink, With<MusicMarker>>,
    mut player_action_event: EventReader<ActionEvent>,
    bindings: Res<InputBindings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    const ACTION: PlayerAction = PlayerAction::ToggleMusic;
    // Check if the action has been called in the event system
    let action_event_fired = player_action_event.read().any(|event| event.0 == ACTION);

    // get the binded keys to the action
    let inputs = bindings.0.get(&ACTION).unwrap_or(&Vec::new()).to_owned();

    if action_event_fired | keyboard_input.any_just_pressed(inputs) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}
