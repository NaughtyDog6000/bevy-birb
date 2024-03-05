use bevy::{prelude::*, render::view::window};

use crate::{components::action_button::ActionButton, systems::player_actions::actions::PlayerAction};


pub fn end_of_game_results(
    mut commands: Commands,
    // mut windows: Query<&mut Window>,
) {
    let root = commands
    .spawn((
        NodeBundle {
            // make it "always on top" by setting the Z index to maximum
            // we want it to be displayed over all other UI
            z_index: ZIndex::Global(i32::MAX),
            style: Style {
                position_type: PositionType::Absolute,

                left: Val::Auto,
                right: Val::Percent(1.0),
                
                top: Val::Auto,
                bottom: Val::Percent(1.0),
                // give it some padding for readability
                ..Default::default()
            },
            ..Default::default()
        },
    ))
    .id();



    // create a button in the bottom right which restarts the game
    let restart_button = commands.spawn((ButtonBundle {
            style: Style {
                width: Val::Px(300.0),
                height: Val::Px(100.0),
                align_self: AlignSelf::FlexEnd,
                justify_content: JustifyContent::Center,
                margin: UiRect::all(Val::Px(20.0)),
                ..Default::default()
            },
            
            ..Default::default()
        }, ActionButton {
            0: PlayerAction::Restart
        })
    ).with_children(|parent| {
        parent.spawn((TextBundle::from_section("Restart", TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.2,0.2,0.2),
            ..Default::default()
        },).with_style(Style {
            align_self: AlignSelf::Center,
            ..Default::default()
        }),
        ));
    }).id();
    
    commands.entity(root).push_children(&[restart_button]);

    
    // panel with score, game length, and other info

    // create a button which uploads the game data to the leaderboard api


    return;
}
