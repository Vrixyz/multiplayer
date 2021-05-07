use crate::*;

pub struct Velocity(pub Vec2);

pub(super) fn velocity(time: Res<Time>, mut vel: Query<(&mut Transform, &Velocity)>) {
    for mut v in vel.iter_mut() {
        v.0.translation += (v.1 .0 * time.delta_seconds()).extend(0.0);
    }
}

pub fn velocity_debug(commander: Query<(&Velocity, &Transform)>, mut lines: ResMut<DebugLines>) {
    let c = match commander.iter().last() {
        Some(it) => it,
        _ => return,
    };
    lines.line_colored(
        c.1.translation,
        c.1.translation + c.0 .0.extend(0.0),
        0.0,
        Color::GREEN,
    );
}
