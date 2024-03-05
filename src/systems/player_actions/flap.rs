use bevy::prelude::*;
use bevy_mod_reqwest::*;

use crate::{
    components::velocity::Velocity, entities::player::Player, systems::score_system::AddScoreEvent,
    InputBindings,
};

use super::actions::{ActionEvent, PlayerAction};

pub fn player_flap(
    mut player_action_event: EventReader<ActionEvent>,
    mut players: Query<&mut Velocity, With<Player>>,
    bindings: Res<InputBindings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    _time: Res<Time>,
    _add_score_event: EventWriter<AddScoreEvent>,
) {
    // check if the flap action has been called in the event system
    let mut flap_event: bool = false;
    {
        for event in player_action_event.read() {
            if event.0 == PlayerAction::Flap {
                flap_event = true;
                break;
            }
        }
    }
    // get the binded keys to the action
    let inputs = bindings
        .0
        .get(&PlayerAction::Flap)
        .unwrap_or(&Vec::new())
        .to_owned();

    if flap_event | keyboard_input.any_just_pressed(inputs) | touch_inputs.any_just_pressed() {
        println!("Flap!");

        for mut velocity in &mut players {
            velocity.y = 4.0;
        }
    }
}
