use bevy::prelude::*;

use crate::{
    components::velocity::Velocity, entities::player::Player, systems::score_system::AddScoreEvent,
};

pub fn player_flap(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touch_inputs: Res<Touches>,
    _time: Res<Time>,
    mut players: Query<&mut Velocity, With<Player>>,
    _add_score_event: EventWriter<AddScoreEvent>,
) {
    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::ArrowUp])
        | touch_inputs.any_just_pressed()
    {
        println!("Flap!");

        for mut velocity in &mut players {
            velocity.y = 4.0;
        }

        // add_score_event.send(AddScoreEvent(5));
    }
}
