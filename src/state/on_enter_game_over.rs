use bevy::prelude::*;

// when the game ends
pub fn on_enter_gameover_state(mut commands: Commands) {
    println!("Game Over");

    commands
        .spawn((NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..default()
        },))
        .with_children(|parent| {
            parent.spawn((
                TextBundle {
                    text: Text::from_section(
                        "GAME OVER!?",
                        TextStyle {
                            color: Color::RED,
                            font_size: 150.0,
                            ..Default::default()
                        },
                    ),
                    ..default()
                },
                Label,
            ));
        });

    return;
}
