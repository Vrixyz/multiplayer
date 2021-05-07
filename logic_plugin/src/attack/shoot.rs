use shared::Id;

use crate::{
    movement::{collisions::*, velocity::Velocity},
    *,
};

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
    mut id_provider: ResMut<IdProvider>,
    mut attacker: Query<(&Transform, &Shooter, &Team, &mut ShootAbility)>,
) {
    for (transform, shooter, team, mut attack) in attacker.iter_mut() {
        match shooter.mode {
            ShooterMode::Shoot(target) => {
                let time_since_startup = time.time_since_startup().as_secs_f32();
                if attack.last_attack + attack.cooldown < time_since_startup {
                    attack.last_attack = time_since_startup;
                    let position = transform.translation;
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
                        .insert(Velocity(
                            (target - position.into()).normalize_or_zero() * 400.0,
                        ));
                }
            }
            ShooterMode::Aim(_) => {}
        };
    }
}