use bevy::prelude::*;
use crate::systems::player_actions::actions::PlayerAction;

#[derive(Component)]
pub struct ActionButton(pub PlayerAction);
// a button has this compoent so the button system can call the associated action when interacted with 