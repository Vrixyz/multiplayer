use bevy::prelude::*;
use bevy_egui::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Menu,
    Loading,
    InGame,
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(EguiPlugin)
            .insert_resource(MenuStateRes {
                ip: "127.0.0.1".to_string(),
            })
            .add_state(AppState::Menu)
            // .add_system(steering_debug.system())
            // .add_system(velocity_debug.system())
            .add_system(menu_ui.system())
            .add_system(in_game.system());
    }
}

#[derive(Default)]
struct MenuStateRes {
    ip: String,
}

fn menu_ui(
    mut state: ResMut<State<AppState>>,
    mut state_network: ResMut<State<multiplayer_plugin::client::State>>,
    mut menuRes: ResMut<MenuStateRes>,
    egui_context: Res<EguiContext>,
) {
    if state.current() != &AppState::Menu {
        return;
    }
    egui::CentralPanel::default().show(egui_context.ctx(), |ui| {
        ui.text_edit_singleline(&mut menuRes.ip);
        if ui.button("Play").clicked() {
            state_network
                .set(multiplayer_plugin::client::State::Connect)
                .unwrap();
            state.set(AppState::InGame).unwrap();
        }
    });
}
fn in_game(mut state: ResMut<State<AppState>>, egui_context: Res<EguiContext>) {
    if state.current() != &AppState::InGame {
        return;
    }
    egui::SidePanel::left("in_game_menu")
        .default_width(200.0)
        .show(egui_context.ctx(), |ui| {
            if ui.button("Leave").clicked() {
                state.set(AppState::Menu).unwrap();
            }
        });
}
