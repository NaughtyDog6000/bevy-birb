use bevy::ecs::component::Component;

#[derive(Component)]
pub enum Collider {
    Circle { radius: f32 },
    Rect { width: f32, height: f32 },
}

impl Default for Collider {
    fn default() -> Self {
        Collider::Circle { radius: 1.0 }
    }
}
