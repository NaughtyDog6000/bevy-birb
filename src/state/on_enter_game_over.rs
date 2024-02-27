use bevy::prelude::*;

// when the game ends
pub fn on_enter_gameover_state(mut commands: Commands) {
    println!("Game Over");

    commands.spawn(
        (TextBundle::from_sections([TextSection::new(
            "GAME OVER!",
            TextStyle {
                color: Color::RED,
                font_size: 150.0,
                ..Default::default()
            },
        )]))
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        }),
    );

    return;
}
