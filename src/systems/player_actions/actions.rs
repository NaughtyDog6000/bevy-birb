use bevy::ecs::event::Event;
use bevy::prelude::*;

use crate::{GameState, InputBindings};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PlayerAction {
    Flap,
    Restart,
    OpenMainMenu,
    ToggleMusic,
    Quit,
}

#[derive(Event)]
pub struct ActionEvent(pub PlayerAction);

pub fn action_open_main_menu(
    mut player_action_event: EventReader<ActionEvent>,
    bindings: Res<InputBindings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    const ACTION: PlayerAction = PlayerAction::OpenMainMenu;
    // Check if the action has been called in the event system
    let flap_event = player_action_event.read().any(|event| event.0 == ACTION);

    // get the binded keys to the action
    let inputs = bindings.0.get(&ACTION).unwrap_or(&Vec::new()).to_owned();

    if flap_event | keyboard_input.any_just_pressed(inputs) | touch_inputs.any_just_pressed() {
        warn!("opening main menu");
        next_state.set(GameState::Menu);
    }
}
