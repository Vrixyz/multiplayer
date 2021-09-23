use bevy_rapier2d::prelude::RigidBodyPosition;
use rapier2d::prelude::RigidBody;
use shared::Id;

use crate::{
    movement::{collisions::*, velocity::Velocity},
    physics::spawn_bullet,
    *,
};

use super::destroy_after::DelayedDestroy;

pub struct Bullet;

#[derive(Clone, PartialEq)]
pub struct Team {
    pub id: usize,
}

pub struct ShootAbility {
    pub cooldown: f32,
    pub last_attack: f32,
}
impl ShootAbility {
    pub fn new(cooldown: f32) -> Self {
        ShootAbility {
            cooldown,
            last_attack: 0.0,
        }
    }
}
pub enum ShooterMode {
    Shoot(Vec2),
    Aim(Vec2),
}
pub struct Shooter {
    pub mode: ShooterMode,
}

pub(super) fn shoot_apply(
    mut commands: Commands,
    time: Res<Time>,
    // FIXME: we might not want to mix ids for bullets and ids for Units
    mut id_provider: ResMut<IdProvider>,
    mut attacker: Query<(&RigidBodyPosition, &Shooter, &Team, &mut ShootAbility)>,
) {
    let time_since_startup = time.time_since_startup().as_secs_f32();
    for (position, shooter, team, mut attack) in attacker.iter_mut() {
        match shooter.mode {
            ShooterMode::Shoot(target) => {
                if attack.last_attack + attack.cooldown < time_since_startup {
                    attack.last_attack = time_since_startup;
                    let position = position.position.translation;
                    spawn_bullet(
                        position.into(),
                        (target - position.into()).normalize_or_zero() * 50.0,
                        time_since_startup + 0.5f32,
                        &mut commands,
                    )
                    .insert(Id(id_provider.new_id()))
                    .insert(Bullet)
                    .insert(team.clone());
                    /*
                    commands
                        .spawn()
                        .insert(Id(id_provider.new_id()))
                        .insert(Transform::from_translation(transform.translation))
                        .insert(Bullet)
                        .insert(team.clone())
                        .insert(CollisionDef {
                            behaviour: CollisionBehaviour::DieAlways,
                        })
                        .insert(CollisionKill)
                        .insert(Shape { radius: 16. })
                        .insert(DelayedDestroy {
                            time_to_destroy: time_since_startup + 0.5f32,
                        })
                        .insert(Velocity(
                            (target - position.into()).normalize_or_zero() * 1000.0,
                        ));*/
                }
            }
            ShooterMode::Aim(_) => {}
        };
    }
}
