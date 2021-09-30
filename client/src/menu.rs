use bevy::prelude::*;
use bevy_egui::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
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
            .add_system(menu_ui.system());
    }
}

#[derive(Default)]
struct MenuStateRes {
    ip: String,
}

fn menu_ui(
    mut state: ResMut<State<AppState>>,
    mut menuRes: ResMut<MenuStateRes>,
    egui_context: Res<EguiContext>,
) {
    if state.current() != &AppState::Menu {
        return;
    }
    egui::CentralPanel::default().show(egui_context.ctx(), |ui| {
        ui.text_edit_singleline(&mut menuRes.ip);
        if ui.button("Play").clicked() {
            state.set(AppState::InGame).unwrap();
        }
    });
}
