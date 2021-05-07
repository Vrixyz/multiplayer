use crate::*;

pub mod shoot;

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // .add_system(steering_debug.system())
            // .add_system(velocity_debug.system())
            .add_system(shoot::shoot_apply.system());
    }
}
