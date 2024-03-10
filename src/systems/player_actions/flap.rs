use bevy::prelude::*;

use crate::{
    components::velocity::Velocity, entities::player::Player, systems::score_system::AddScoreEvent,
    InputBindings,
};

use super::actions::{ActionEvent, PlayerAction};

pub fn player_flap(
    mut players: Query<&mut Velocity, With<Player>>,
    mut player_action_event: EventReader<ActionEvent>,
    bindings: Res<InputBindings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    _time: Res<Time>,
    _add_score_event: EventWriter<AddScoreEvent>,
) {
    const ACTION: PlayerAction = PlayerAction::Flap;
    // Check if the action has been called in the event system
    let action_event_fired = player_action_event.read().any(|event| event.0 == ACTION);

    // get the binded keys to the action
    let inputs = bindings.0.get(&ACTION).unwrap_or(&Vec::new()).to_owned();

    if action_event_fired
        | keyboard_input.any_just_pressed(inputs)
        | touch_inputs.any_just_pressed()
    {
        println!("Flap!");

        for mut velocity in &mut players {
            velocity.y = 4.0;
        }
    }
}
