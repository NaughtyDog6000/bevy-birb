use bevy::{app::AppExit, prelude::*};

use crate::InputBindings;

use super::actions::{ActionEvent, PlayerAction};

pub fn action_quit(
    mut player_action_event: EventReader<ActionEvent>,
    bindings: Res<InputBindings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>, // touch_inputs: Res<Touches>,
) {
    const ACTION: PlayerAction = PlayerAction::Quit;
    // Check if the action has been called in the event system
    let action_event_fired = player_action_event.read().any(|event| event.0 == ACTION);

    // get the binded keys to the action
    let inputs = bindings.0.get(&ACTION).unwrap_or(&Vec::new()).to_owned();

    if action_event_fired | keyboard_input.any_just_pressed(inputs) {
        exit.send(AppExit);
    }
}
