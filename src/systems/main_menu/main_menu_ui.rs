use bevy::{app::AppExit, prelude::*};
use bevy_egui::{egui, EguiContexts};

use crate::{
    systems::player_actions::actions::{ActionEvent, PlayerAction},
    GameState,
};

#[derive(Default, Resource)]
pub struct MainMenuUIState {
    // sliders, text boxes etc (Not buttons)
}

pub fn draw_main_menu_ui(
    mut contexts: EguiContexts,
    // mut ui_state: ResMut<MainMenuUIState>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: EventWriter<AppExit>,
    mut player_action_event: EventWriter<ActionEvent>,
) {
    let ctx = contexts.ctx_mut();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Main Menu");
        ui.add(egui::Hyperlink::from_label_and_url(
            "Github Repo for this project",
            "https://github.com/NaughtyDog6000/bevy-birb",
        ));
        egui::warn_if_debug_build(ui);

        ui.separator();

        if ui.button("Play").clicked() {
            warn!("PRESSED START!");
            next_state.set(GameState::InGame);
        }

        if ui.button("Quit").clicked() {
            exit.send(AppExit);
        }

        if ui.button("Toggle Music").clicked() {
            player_action_event.send(ActionEvent(PlayerAction::ToggleMusic));
        }

        ui.label("Welcome to bevy birb :)");

        // ui.heading("Heading");
    });
}
