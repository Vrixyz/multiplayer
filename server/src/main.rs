use bevy::{app::ScheduleRunnerSettings, prelude::*};
use logic_plugin::{
    attack::{
        shoot::{Bullet, ShootAbility, Shooter, ShooterMode, Team},
        AttackPlugin,
    },
    movement::{
        collisions::{CollisionBehaviour, CollisionDef, Shape},
        velocity::Velocity,
        MovementPlugin, SteerDirection, SteeringManager,
    },
    physics::{create_player, PhysicsPlugin, RapierConfiguration},
    IdProvider, Unit,
};
use multiplayer_plugin::server::*;
use shared::{network::udp_server::ComServer, Id, ServerMessage};
use std::time::Duration;

const BULLET_SIZE: f32 = 16_f32;

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / 30.0,
        )))
        .insert_resource(IdProvider::default())
        .add_plugin(MultiplayerServerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(MovementPlugin)
        .add_plugin(AttackPlugin)
        .add_system_to_stage(CoreStage::PreUpdate, handle_messages.system())
        .add_system_to_stage(CoreStage::PostUpdate, send_update.system())
        .run();
}

fn send_update(
    rapier_configuration: Res<RapierConfiguration>,
    mut messages_to_send: ResMut<MessagesToSend>,
    com: ResMut<ComServer>,
    units: Query<(&shared::Id, &Transform, &Team, &Unit, &Shape)>,
    bullets: Query<(&shared::Id, &Transform, &Team, &Bullet)>,
) {
    let mut world = shared::World {
        entities: vec![],
        scale: rapier_configuration.scale,
    };
    for (id, transform, team, _, shape) in units.iter() {
        world.entities.push(shared::Entity {
            position: shared::Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            size: shape.radius,
            id: id.0 as usize,
            team: team.id,
        })
    }
    for (e, transform, team, _) in bullets.iter() {
        world.entities.push(shared::Entity {
            position: shared::Vec2 {
                x: transform.translation.x,
                y: transform.translation.y,
            },
            size: BULLET_SIZE,
            id: e.0 as usize,
            team: team.id,
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
    mut id_provider: ResMut<IdProvider>,
    mut units: Query<(&mut SteerDirection, &mut Unit, Option<&mut Shooter>)>,
) {
    while let Some((c, m)) = messages_to_read.pop() {
        if let Some(mut unit) = units.iter_mut().find(|u| u.1.client_id == c.id) {
            apply_command(&m.command, &mut unit.0, &mut unit.2.as_deref_mut());
        } else {
            let steering_manager = SteeringManager {
                steering_target: Vec2::ZERO,
            };
            let mut seeker = SteerDirection {
                direction: Vec2::ZERO,
            };
            let mut shooter = Shooter {
                mode: ShooterMode::Aim(Vec2::ZERO),
            };
            apply_command(&m.command, &mut seeker, &mut Some(&mut shooter));
            create_player(
                &mut commands,
                &mut id_provider,
                seeker,
                steering_manager,
                c,
                shooter,
            );
        };
    }
}

fn apply_command(
    command: &shared::Command,
    seeker: &mut SteerDirection,
    shooter: &mut Option<&mut Shooter>,
) {
    match command {
        shared::Command::MoveDirection(move_command) => {
            seeker.direction = Vec2::new(move_command.x, move_command.y).normalize_or_zero();
        }
        shared::Command::Shoot(target) => {
            if let Some(shooter) = shooter {
                shooter.mode = ShooterMode::Shoot(Vec2::new(target.x, target.y));
            }
        }
        shared::Command::Aim(target) => {
            if let Some(shooter) = shooter {
                shooter.mode = ShooterMode::Aim(Vec2::new(target.x, target.y));
            }
        }
    }
}
