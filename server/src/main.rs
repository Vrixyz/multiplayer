use bevy::{app::ScheduleRunnerSettings, prelude::*};
use multiplayer_plugin::server::*;
use shared::{network::com_server::ComServer, ServerMessage};
use std::time::Duration;

struct Unit {
    pub client_id: usize,
}

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 50.0,
        )))
        .add_plugin(MultiplayerServerPlugin)
        .add_system(handle_messages.system())
        .add_system(send_update.system())
        .run();
}

fn send_update(
    mut messages_to_send: ResMut<MessagesToSend>,
    com: ResMut<ComServer>,
    units: Query<(Entity, &Transform, &Unit)>,
) {
    let mut world = shared::World { entities: vec![] };
    for (e, transform, unit) in units.iter() {
        world.entities.push(shared::Entity {
            position: shared::Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            velocity: shared::Vec2::default(),
            id: e.id() as usize,
            team: unit.client_id,
        })
    }
    for (_, c) in com.clients_iter() {
        messages_to_send.push((
            c.clone(),
            ServerMessage {
                world: world.clone(),
            },
        ));
    }
}

fn handle_messages(
    mut commands: Commands,
    mut messages_to_read: ResMut<MessagesToRead>,
    mut units: Query<(&mut Transform, &mut Unit)>,
) {
    while let Some((c, m)) = messages_to_read.pop() {
        if let Some(mut unit) = units.iter_mut().find(|u| u.1.client_id == c.id) {
            &unit.0.translation;
            apply_command(&m.command, &mut unit.0.translation, &mut unit.1);
            &unit.0.translation;
        } else {
            let mut transform = Transform::default();
            let mut unit = Unit { client_id: c.id };
            apply_command(&m.command, &mut transform.translation, &mut unit);
            commands.spawn().insert(transform).insert(unit);
        };
    }
}

fn apply_command(command: &shared::Command, translation: &mut Vec3, unit: &mut Unit) {
    match command {
        shared::Command::Move(move_command) => {
            translation.x = move_command.x;
            translation.y = move_command.y;
        }
        shared::Command::Shoot(_) => {}
    }
}
