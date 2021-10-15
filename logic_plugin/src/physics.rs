use bevy::{ecs::system::EntityCommands, math::Vec2, prelude::*};
use bevy_rapier2d::{
    na::{Isometry2, Vector2},
    prelude::*,
};
use rapier2d::prelude::RigidBody;
use shared::Id;

pub use bevy_rapier2d::physics::RapierConfiguration;

use crate::{
    attack::{
        destroy_after::DelayedDestroy,
        shoot::{ShootAbility, Shooter, Team},
        Health,
    },
    movement::{
        self,
        collisions::{CollisionBehaviour, CollisionDef, CollisionKill},
        SteerDirection, SteeringManager,
    },
    IdProvider, Unit,
};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_startup_system(setup_physics.system());
    }
}

pub fn setup_physics(mut commands: Commands, mut configuration: ResMut<RapierConfiguration>) {
    configuration.gravity = Vector2::zeros();
    configuration.scale = 1f32;

    /*
     * Ground
     */
    let ground_size = 20.0;

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.0),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderPositionSync::Discrete);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.),
        position: Isometry2::new(
            [ground_size, ground_size].into(),
            std::f32::consts::FRAC_PI_2,
        )
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.),
        position: Isometry2::new(
            [-ground_size, ground_size].into(),
            std::f32::consts::FRAC_PI_2,
        )
        .into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(ground_size, 1.),
        position: Isometry2::new([0.0, ground_size * 2.0].into(), 0.0).into(),
        ..Default::default()
    };
    commands
        .spawn_bundle(collider)
        .insert(ColliderDebugRender::default())
        .insert(ColliderPositionSync::Discrete);

    /*
     * Create the cubes
     */
    let num = 5;
    let rad = 0.5;

    let shift = rad * 2.0;
    let centerx = shift * (num as f32) / 2.0;
    let centery = shift / 2.0;
    let mut color = 0;

    for i in 0..num {
        for j in 0usize..num {
            let x = i as f32 * shift - centerx;
            let y = j as f32 * shift + centery + 2.0;
            color += 1;

            // Build the rigid body.
            let body = RigidBodyBundle {
                position: [x, y].into(),
                ..Default::default()
            };
            let collider = ColliderBundle {
                shape: ColliderShape::cuboid(rad, rad),
                ..Default::default()
            };
            commands
                .spawn_bundle(body)
                .insert_bundle(collider)
                .insert(Health {
                    base: 1.0,
                    current: 1.0,
                })
                .insert(ColliderDebugRender::with_id(color))
                .insert(ColliderPositionSync::Discrete);
        }
    }
}

pub fn spawn_bullet<'a, 'b>(
    origin: Vec2,
    speed: Vec2,
    time_to_destroy: f32,
    commands: &'b mut Commands<'a>,
) -> EntityCommands<'a, 'b> {
    let collider_size = 0.5;

    let mut body = RigidBodyBundle {
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let direction = speed.normalize();
    body.velocity.linvel = speed.into();
    body.mass_properties.flags = RigidBodyMassPropsFlags::ROTATION_LOCKED;
    let spawn_position = origin + direction * 2.0;
    body.position = spawn_position.into();
    let mut ret = commands.spawn();
    ret.insert_bundle(body)
        .insert_bundle(ColliderBundle {
            flags: (ActiveEvents::CONTACT_EVENTS).into(),
            position: [0.0, 0.0].into(),
            shape: ColliderShape::ball(collider_size / 2.0),
            ..Default::default()
        })
        .insert(movement::collisions::Shape { radius: 16. })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(0))
        .insert(CollisionKill)
        .insert(CollisionDef {
            behaviour: CollisionBehaviour::DieAlways,
        })
        .insert(DelayedDestroy { time_to_destroy });
    ret
}

pub fn create_player(
    commands: &mut Commands,
    id_provider: &mut ResMut<IdProvider>,
    seeker: SteerDirection,
    steering_manager: SteeringManager,
    c: shared::network::udp_server::Client,
    shooter: Shooter,
) {
    let unit = Unit { client_id: c.id };

    let mut body = RigidBodyBundle {
        ccd: RigidBodyCcd {
            ccd_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    };

    body.mass_properties.flags = RigidBodyMassPropsFlags::ROTATION_LOCKED;
    body.position = RigidBodyPosition::default();
    commands
        .spawn()
        .insert_bundle(body)
        .insert_bundle(ColliderBundle {
            flags: (ActiveEvents::CONTACT_EVENTS).into(),
            position: [0.0, 0.0].into(),
            shape: ColliderShape::ball(1f32 / 2.0),
            ..Default::default()
        })
        .insert(RigidBodyPositionSync::Discrete)
        .insert(Id(id_provider.new_id()))
        .insert(movement::collisions::Shape { radius: 32f32 })
        .insert(CollisionDef {
            behaviour: CollisionBehaviour::DieVersusKill,
        })
        .insert(seeker)
        .insert(steering_manager)
        .insert(Team { id: c.id })
        .insert(shooter)
        .insert(ShootAbility::new(0.1))
        .insert(unit);
}
