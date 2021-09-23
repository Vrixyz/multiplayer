use std::cmp::min;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

use crate::attack::shoot::Team;

use super::velocity::Velocity;

pub struct Shape {
    pub radius: f32,
}
pub enum CollisionBehaviour {
    DieAlways,
    DieVersusKill,
    NeverDie,
}
pub struct CollisionKill;

pub struct CollisionDef {
    pub behaviour: CollisionBehaviour,
}

pub(super) fn collisions_death(
    mut commands: Commands,
    mut collision_checks: QuerySet<(
        Query<(
            Entity,
            &Transform,
            &Shape,
            &Team,
            &CollisionDef,
            Option<&CollisionKill>,
        )>,
        Query<(
            Entity,
            &Transform,
            &Shape,
            &Team,
            &CollisionDef,
            Option<&CollisionKill>,
        )>,
        Query<(Entity, &mut Transform)>,
    )>,
) {
    let mut positions_fixes = vec![];
    let mut first_check = 1;
    for (e1, t1, s1, team1, def1, kill1) in collision_checks.q0().iter() {
        for (e2, t2, s2, team2, def2, kill2) in collision_checks.q1().iter().skip(first_check) {
            if team1.id == team2.id {
                continue;
            }
            let distance = t1.translation.distance(t2.translation);
            let min_distance = s1.radius + s2.radius;
            if distance < min_distance {
                let mut move1 = true;
                let mut move2 = true;

                // TODO: both match arms can be put in a same function.
                match def1.behaviour {
                    CollisionBehaviour::DieAlways => {
                        commands.entity(e1).despawn();
                        move1 = false;
                    }
                    CollisionBehaviour::DieVersusKill => {
                        if kill2.is_some() {
                            commands.entity(e1).despawn();
                            move1 = false;
                        }
                    }
                    CollisionBehaviour::NeverDie => {}
                }
                match def2.behaviour {
                    CollisionBehaviour::DieAlways => {
                        commands.entity(e2).despawn();
                        move2 = false;
                    }
                    CollisionBehaviour::DieVersusKill => {
                        if kill1.is_some() {
                            commands.entity(e2).despawn();
                            move2 = false;
                        }
                    }
                    CollisionBehaviour::NeverDie => {}
                }
                if move1 && move2 {
                    positions_fixes.push((
                        e1,
                        t2.translation
                            + (t1.translation - t2.translation).normalize() * min_distance,
                    ));
                    positions_fixes.push((
                        e2,
                        t1.translation
                            + (t2.translation - t1.translation).normalize() * min_distance,
                    ));
                }
            }
        }
        first_check += 1;
    }
    for (e, pos) in positions_fixes {
        if let Ok(mut to_fix) = collision_checks.q2_mut().get_component_mut::<Transform>(e) {
            to_fix.translation = pos;
        }
    }
}
