use crate::*;
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
            .add_system(collisions_border.system().after("velocity_update"))
            .add_system(collisions_death.system().after("velocity_update"));
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

pub const MAX_SPEED: f32 = 100.0;

impl SteeringManager {
    pub fn do_seek(current_position: Vec2, target: Vec2, current_speed: Vec2, mass: f32) -> Vec2 {
        let mut desired = target - current_position;
        let distance = desired.length();

        desired = desired.normalize_or_zero();
        let slowing_radius = 50.0;

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
    mut steering_managers: Query<(&mut SteeringManager, &Transform, &Velocity, &Seeker)>,
) {
    for (mut steer, transform, velocity, seeker) in steering_managers.iter_mut() {
        steer.steering_target = SteeringManager::do_seek(
            transform.translation.into(),
            seeker.target,
            velocity.0,
            20.0,
        );
    }
}
fn direction_update(
    mut steering_managers: Query<(&mut SteeringManager, &Velocity, &SteerDirection)>,
) {
    for (mut steer, velocity, seeker) in steering_managers.iter_mut() {
        steer.steering_target =
            SteeringManager::do_desired(seeker.direction * MAX_SPEED, velocity.0, 200.0);
    }
}
pub fn steering_targets_influence(
    time: Res<Time>,
    mut steering_managers: Query<(&Transform, &mut Velocity, &mut SteeringManager)>,
) {
    for (_transform, mut velocity, mut manager) in steering_managers.iter_mut() {
        manager.steering_target = manager.steering_target.clamp_length_max(100.0);

        velocity.0 += manager.steering_target;
        velocity.0.clamp_length_max(MAX_SPEED);
    }
}

pub fn steering_debug(
    commander: Query<(&SteeringManager, &Transform, &Velocity)>,
    mut lines: ResMut<DebugLines>,
) {
    let c = match commander.iter().last() {
        Some(it) => it,
        _ => return,
    };
    let start = c.1.translation + c.2 .0.extend(0.0);
    lines.line_colored(
        start,
        start + c.0.steering_target.extend(0.0),
        0.0,
        Color::RED,
    );
}
