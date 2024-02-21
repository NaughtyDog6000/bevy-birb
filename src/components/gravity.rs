use bevy::ecs::{component::Component};


#[derive(Component)]
pub struct Gravity {
    pub x: f32,
    pub y: f32,
}
