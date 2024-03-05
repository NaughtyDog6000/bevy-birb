use crate::systems::player_actions::actions::PlayerAction;
use bevy::prelude::*;

#[derive(Component)]
pub struct ActionButton(pub PlayerAction);
// a button has this compoent so the button system can call the associated action when interacted with
