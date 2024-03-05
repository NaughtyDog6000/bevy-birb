use bevy::prelude::*;

pub fn end_of_game_results(
    mut commands: Commands,

) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn(ButtonBundle {
                style: Style {
                    width: Val::Px(300.0),
                    height: Val::Px(300.0),
                    justify_content: JustifyContent::Center,
                    align_content: AlignContent::Center,
                    margin: UiRect::all(Val::Px(20.0)),
                    ..Default::default()
                },
                ..Default::default()
            }).with_children(|parent| {
            parent.spawn(TextBundle::from_section("Button", TextStyle {
                font_size: 40.0,
                color: Color::rgb(1.0, 1.0, 0.0),
                ..Default::default()
            },));
        });
    });
    
    // panel with score, game length, and other info

    // create a button in the bottom right which restarts the game
    // create a button which uploads the game data to the leaderboard api


    return;
}
