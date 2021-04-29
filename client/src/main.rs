use bevy::prelude::*;
use multiplayer_plugin::client::*;
use shared::{ClientMessage, Command, Vec2};

#[derive(Debug)]
struct Unit {
    id: usize,
}

struct UnitMaterial {
    pub mat: Handle<ColorMaterial>,
}
struct MainCamera;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MultiplayerClientPlugin)
        .add_startup_system(init_assets.system())
        .add_system(handle_messages.system())
        .add_system(send_clicks.system())
        //.add_system(display_world.system())
        .run();
}
fn init_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("icon.png");
    let mat = materials.add(texture_handle.into());
    commands.insert_resource(UnitMaterial { mat });
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

fn send_clicks(
    mut input: EventReader<CursorMoved>,
    mut messages_to_send: ResMut<MessagesToSend>,
    q_camera: Query<&Transform, With<MainCamera>>,
    wnds: Res<Windows>,
) {
    let pos = match input.iter().last() {
        None => return,
        Some(pos) => {
            // From https://bevy-cheatbook.github.io/cookbook/cursor2world.html?highlight=world#2d-games
            let wnd = wnds.get(pos.id).unwrap();
            let size = bevy::math::Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let p = pos.position - size / 2.0;
            let camera_transform = q_camera.iter().next().unwrap();
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            Vec2 {
                x: pos_wld.x,
                y: pos_wld.y,
            }
        }
    };

    let value = ClientMessage {
        command: Command::Move(pos),
    };
    messages_to_send.push(value);
}

fn handle_messages(
    mut commands: Commands,
    mut messages_to_read: ResMut<MessagesToRead>,
    mut query: Query<(Entity, &mut Transform, &mut Unit)>,
    assets: Res<UnitMaterial>,
) {
    match messages_to_read.pop() {
        None => return,
        Some(mut m) => {
            let mut updated_units = vec![];
            for mut u in query.iter_mut() {
                if let Some(pos_updated_value) = m
                    .world
                    .entities
                    .iter()
                    .position(|to_update| to_update.id == u.2.id)
                {
                    let updated_value = &m.world.entities[pos_updated_value];
                    u.1.translation.x = updated_value.position.x;
                    u.1.translation.y = updated_value.position.y;
                    dbg!(u);
                    updated_units.push(pos_updated_value);
                } else {
                    commands.entity(dbg!(u.0)).despawn();
                }
            }
            m.world.entities.retain(|e| !updated_units.contains(&e.id));
            for new_entity in m.world.entities {
                commands
                    .spawn()
                    .insert(dbg!(Unit { id: new_entity.id }))
                    .insert_bundle(SpriteBundle {
                        material: assets.mat.clone(),
                        ..Default::default()
                    })
                    .insert(Transform::from_translation(Vec3::new(
                        new_entity.position.x,
                        new_entity.position.y,
                        0.0,
                    )));
            }
        }
    }
}
