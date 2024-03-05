use bevy::ecs::event::Event;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum PlayerAction {
    Flap,
    Restart,
    Quit,
}

#[derive(Event)]
pub struct ActionEvent(pub PlayerAction);
