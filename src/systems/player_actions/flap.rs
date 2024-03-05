use bevy::prelude::*;
use bevy_mod_reqwest::*;

use crate::{
    components::velocity::Velocity,
    entities::player::Player,
    systems::score_system::AddScoreEvent,
    InputBindings,
    // InputBindings,
};

use super::actions::{ActionEvent, PlayerAction};

pub fn player_flap(
    mut player_action_event: EventReader<ActionEvent>,
    mut players: Query<&mut Velocity, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    _time: Res<Time>,
    _add_score_event: EventWriter<AddScoreEvent>,
) {
    // check if the flap action has been called in the event system
    let flap_event: bool = false; 
    {
        for event in player_action_event.read() {
            if event.0 == PlayerAction::Flap {
                break;
            } 
        }
    }

    if flap_event 
    | keyboard_input.any_just_pressed([KeyCode::KeyR, KeyCode::Backspace])
    | touch_inputs.any_just_pressed() {
        println!("Flap!");

        for mut velocity in &mut players {
            velocity.y = 4.0;
        }
        // add_score_event.send(AddScoreEvent(5));
    }
}
