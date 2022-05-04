use bevy_rapier3d::dynamics::{
    AdditionalMassProperties, Ccd, Damping, Dominance, ExternalForce, ExternalImpulse,
    GravityScale, ImpulseJoint, LockedAxes, MassProperties, MultibodyJoint,
    RapierImpulseJointHandle, RapierMultibodyJointHandle, RapierRigidBodyHandle, RigidBody,
    Sleeping, TransformInterpolation, Velocity,
};
use bevy_rapier3d::geometry::{
    ActiveCollisionTypes, ActiveEvents, ActiveHooks, Collider, ColliderMassProperties,
    ColliderScale, CollisionGroups, Friction, RapierColliderHandle, Restitution, Sensor,
    SolverGroups,
};
use bevy_rapier3d::pipeline::{
    CollisionEvent, PhysicsHooksWithQueryInstance, PhysicsHooksWithQueryResource,
};
use bevy_rapier3d::plugin::configuration::{SimulationToRenderTime};
use bevy_rapier3d::plugin::{RapierConfiguration, RapierContext};
use bevy_rapier3d::prelude::CollidingEntities;
use bevy_rapier3d::utils;
use bevy::ecs::query::WorldQuery;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use rapier3d::prelude::*;
use std::collections::HashMap;

pub fn step_simulation<PhysicsHooksData: 'static + WorldQuery + Send + Sync>(
    mut context: ResMut<RapierContext>,
    config: Res<RapierConfiguration>,
    hooks: Res<PhysicsHooksWithQueryResource<PhysicsHooksData>>,
    (time, mut sim_to_render_time): (Res<Time>, ResMut<SimulationToRenderTime>),
    events: EventWriter<CollisionEvent>,
    hooks_data: Query<PhysicsHooksData>,
    interpolation_query: Query<(&RapierRigidBodyHandle, &mut TransformInterpolation)>,
) {
    let context = &mut *context;

    if config.physics_pipeline_active {
        let hooks_instance = PhysicsHooksWithQueryInstance {
            user_data: hooks_data,
            hooks: &*hooks.0,
        };

        context.step_simulation(
            config.gravity,
            config.timestep_mode,
            Some(events),
            &hooks_instance,
            &*time,
            &mut *sim_to_render_time,
            Some(interpolation_query),
        );
        context.deleted_colliders.clear();
    } else {
        context.propagate_modified_body_positions_to_colliders();
    }

    if config.query_pipeline_active {
        context.update_query_pipeline();
    }
}
