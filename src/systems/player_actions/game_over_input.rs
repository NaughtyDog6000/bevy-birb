use bevy::prelude::*;

use crate::{GameState, InputBindings};

pub fn game_over_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.any_just_pressed([KeyCode::KeyR, KeyCode::Backspace])
        | touch_inputs.any_just_pressed()

    {
        next_state.set(GameState::InGame);
    }
}
