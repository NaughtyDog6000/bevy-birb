use bevy::ecs::system::Commands;
use bevy::prelude::*;
use bevy::window::WindowMode;

pub fn toggle_fullscreen_system(
    _commands: Commands,
    mut windows: Query<&mut Window>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if input.just_pressed(KeyCode::F11) {
        if window.mode != WindowMode::BorderlessFullscreen {
            window.mode = WindowMode::BorderlessFullscreen;
        } else {
            window.mode = WindowMode::Windowed;
        }
    }
}
