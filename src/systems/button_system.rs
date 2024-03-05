use bevy::prelude::*;

use crate::components::action_button::ActionButton;

use super::player_actions::actions::ActionEvent;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Children, Entity, &ActionButton),
        (Changed<Interaction>, With<Button>, With<ActionButton>),
    >,
    // mut text_query: Query<&mut Text>,
    mut background_query: Query<&mut BackgroundColor>,
    mut player_action_event: EventWriter<ActionEvent>,
) {
    for (interaction, _children, entity, btn_action) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        let mut background_colour = background_query.get_mut(entity).unwrap();
        match *interaction {
            Interaction::Pressed => {
                player_action_event.send(ActionEvent(btn_action.0));
                // text.sections[0].value = "Press".to_string();
                background_colour.0 = Color::RED;
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                background_colour.0 = Color::GRAY;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                background_colour.0 = Color::WHITE;
            }
        }
    }
}
