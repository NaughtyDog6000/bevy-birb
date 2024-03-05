use bevy::prelude::*;
use std::collections::HashMap;

use bevy::ecs::event::Event;

use crate::InputBindings;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PlayerAction {
    Flap,
    Restart,
    Quit,

}

#[derive(Event)]
pub struct ActionEvent(pub PlayerAction);