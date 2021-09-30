use bevy::{math::Vec2, prelude::*};
use multiplayer_plugin::client::*;
use shared::{ClientMessage, Command, Id};

mod menu;

struct UnitMaterial {
    pub mat: Handle<ColorMaterial>,
}
struct MainCamera;
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(MultiplayerClientPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_startup_system(init_assets.system())
        .add_system_to_stage(CoreStage::PreUpdate, handle_messages.system())
        .add_system(input_aim_system.system())
        .add_system(input_move_system.system())
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

fn input_aim_system(
    window: Res<Windows>,
    mut messages_to_send: ResMut<MessagesToSend>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut ev_cursor: EventReader<CursorMoved>,
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    if let Some(pos) = ev_cursor.iter().last() {
        let camera_transform = q_camera.iter().next().unwrap();
        let wnd = window.get(pos.id).unwrap();
        let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // the default orthographic projection is in pixels from the center;
        // just undo the translation
        let p = pos.position - size / 2.0;

        // apply the camera transform
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);

        if mouse_button_input.pressed(MouseButton::Left) {
            messages_to_send.push(ClientMessage {
                command: Command::Shoot(shared::Vec2 {
                    x: pos_wld.x / 32.0,
                    y: pos_wld.y / 32.0,
                }),
            });
        } else {
            messages_to_send.push(ClientMessage {
                command: Command::Aim(shared::Vec2 {
                    x: pos_wld.x / 32.0,
                    y: pos_wld.y / 32.0,
                }),
            });
        }
    }
}

pub fn input_move_system(
    mut messages_to_send: ResMut<MessagesToSend>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.is_changed() {
        let mut movement = Vec2::splat(0.0);
        if keyboard_input.pressed(KeyCode::Z) {
            movement += Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::S) {
            movement += -Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::D) {
            movement += Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::Q) {
            movement += -Vec2::X;
        }
        movement = movement.normalize_or_zero();
        let movement = shared::Vec2 {
            x: movement.x,
            y: movement.y,
        };
        messages_to_send.push(ClientMessage {
            command: Command::MoveDirection(movement),
        });
    }
}

fn handle_messages(
    mut commands: Commands,
    mut messages_to_read: ResMut<MessagesToRead>,
    mut query: Query<(Entity, &mut Transform, &mut Id)>,
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
                    .position(|to_update| to_update.id == u.2 .0)
                {
                    let updated_value = &m.world.entities[pos_updated_value];

                    u.1.translation.x = updated_value.position.x * 32.0;
                    u.1.translation.y = updated_value.position.y * 32.0;
                    updated_units.push(updated_value.id);
                } else {
                    commands.entity(u.0).despawn();
                }
            }
            m.world.entities.retain(|e| !updated_units.contains(&e.id));
            for new_entity in m.world.entities {
                println!(
                    "new entity {} to {};{}",
                    new_entity.id, new_entity.position.x, new_entity.position.y,
                );
                commands
                    .spawn()
                    .insert(Id(new_entity.id))
                    .insert_bundle(SpriteBundle {
                        material: assets.mat.clone(),
                        sprite: Sprite::new(Vec2::splat(new_entity.size * 2f32)),
                        ..Default::default()
                    })
                    .insert(Transform::from_translation(Vec3::new(
                        new_entity.position.x * 32.0,
                        new_entity.position.y * 32.0,
                        0.0,
                    )));
            }
        }
    }
}
