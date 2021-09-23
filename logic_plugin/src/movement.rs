use crate::*;
use bevy_rapier2d::prelude::{Real, RigidBodyPosition, RigidBodyVelocity, Vector};
use collisions::*;
use velocity::*;

pub mod collisions;
pub mod velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .add_system(steering_debug.system())
            // .add_system(velocity_debug.system())
            .add_system(seekers_update.system().before("steering_update"))
            .add_system(direction_update.system().before("steering_update"))
            .add_system(steering_targets_influence.system().label("steering_update"))
            .add_system(
                velocity
                    .system()
                    .after("steering_update")
                    .label("velocity_update"),
            )
            //.add_system(collisions_death.system().after("velocity_update"))
            ;
    }
}

pub struct SteeringManager {
    pub steering_target: Vec2,
}

pub struct Seeker {
    pub target: Vec2,
}
pub struct SteerDirection {
    pub direction: Vec2,
}

pub const MAX_SPEED: f32 = 10.0;

impl SteeringManager {
    pub fn do_seek(current_position: Vec2, target: Vec2, current_speed: Vec2, mass: f32) -> Vec2 {
        let mut desired = target - current_position;
        let distance = desired.length();

        desired = desired.normalize_or_zero();
        let slowing_radius = 0.1;

        if distance <= slowing_radius {
            desired *= MAX_SPEED * distance / slowing_radius;
        } else {
            desired *= MAX_SPEED;
        }
        Self::do_desired(desired, current_speed, mass)
    }
    pub fn do_desired(desired: Vec2, current_speed: Vec2, mass: f32) -> Vec2 {
        let force = desired - current_speed;

        force / mass
    }
}

fn seekers_update(
    mut steering_managers: Query<(
        &mut SteeringManager,
        &RigidBodyPosition,
        &RigidBodyVelocity,
        &Seeker,
    )>,
) {
    for (mut steer, position, velocity, seeker) in steering_managers.iter_mut() {
        steer.steering_target = SteeringManager::do_seek(
            position.position.translation.into(),
            seeker.target,
            velocity.linvel.into(),
            20.0,
        );
    }
}
fn direction_update(
    mut steering_managers: Query<(&mut SteeringManager, &RigidBodyVelocity, &SteerDirection)>,
) {
    for (mut steer, velocity, seeker) in steering_managers.iter_mut() {
        steer.steering_target =
            SteeringManager::do_desired(seeker.direction * MAX_SPEED, velocity.linvel.into(), 10.0);
    }
}
pub fn steering_targets_influence(
    time: Res<Time>,
    mut steering_managers: Query<(
        &RigidBodyPosition,
        &mut RigidBodyVelocity,
        &mut SteeringManager,
    )>,
) {
    for (_position, mut velocity, mut manager) in steering_managers.iter_mut() {
        manager.steering_target = manager.steering_target.clamp_length_max(100.0);

        let target: Vector<Real> = manager.steering_target.into();
        velocity.linvel += target;
        let glam_vector: Vec2 = velocity.linvel.into();
        glam_vector.clamp_length_max(MAX_SPEED);
        velocity.linvel = glam_vector.into();
    }
}

pub fn steering_debug(
    commander: Query<(&SteeringManager, &RigidBodyPosition, &RigidBodyVelocity)>,
    mut lines: ResMut<DebugLines>,
) {
    let c = match commander.iter().last() {
        Some(it) => it,
        _ => return,
    };
    let vel: Vec2 = c.2.linvel.into();
    let pos: Vec2 = c.1.position.translation.into();
    let start: Vec3 = (pos + vel).extend(0.0);
    lines.line_colored(
        start,
        start + c.0.steering_target.extend(0.0),
        0.0,
        Color::RED,
    );
}
