use bevy::prelude::*;

use crate::{systems::player_actions::actions::{ActionEvent, PlayerAction}, GameState, InputBindings};

pub fn game_over_input(
    mut player_action_event: EventReader<ActionEvent>,
    bindings: Res<InputBindings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // check if the flap action has been called in the event system
    let mut restart_event: bool = false; 
    {
        for event in player_action_event.read() {
            if event.0 == PlayerAction::Restart {
                restart_event = true;
                break;
            } 
        }
    }
    // get the binded keys to the action
    let inputs = bindings.0.get(&PlayerAction::Restart).unwrap_or(&Vec::new()).to_owned();


    if restart_event
        | keyboard_input.any_just_pressed(inputs)
        | touch_inputs.any_just_pressed()

    {
        next_state.set(GameState::InGame);
    }
}
