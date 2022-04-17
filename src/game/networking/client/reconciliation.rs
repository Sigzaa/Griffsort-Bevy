use super::components::*;
use crate::game::components::{filters::*, player_data::*, *};

use bevy_rapier3d::physics::{
    ColliderComponentsQuerySet, ColliderComponentsSet, ColliderPositionSync, ComponentSetQueryMut,
    EventQueue, IntoEntity, IntoHandle, JointBuilderComponent, JointHandleComponent,
    JointsEntityMap, ModificationTracker, PhysicsHooksWithQueryInstance,
    PhysicsHooksWithQueryObject, QueryComponentSetMut, RapierConfiguration,
    RigidBodyComponentsQuerySet, RigidBodyComponentsSet, RigidBodyPositionSync,
    SimulationToRenderTime, TimestepMode,
};

use bevy_rapier3d::physics::wrapper::{
    ColliderBroadPhaseDataComponent, ColliderChangesComponent, ColliderMassPropsComponent,
    ColliderParentComponent, ColliderPositionComponent, ColliderShapeComponent,
    RigidBodyCcdComponent, RigidBodyChangesComponent, RigidBodyCollidersComponent,
    RigidBodyIdsComponent, RigidBodyMassPropsComponent, RigidBodyPositionComponent,
};
use bevy_rapier3d::prelude::{ContactEvent, IntersectionEvent};
use bevy_rapier3d::rapier::data::ComponentSetOption;

use bevy::ecs::entity::Entities;
use bevy::ecs::query::{QueryState, WorldQuery};
use bevy::prelude::*;
use bevy_rapier3d::rapier::pipeline::QueryPipeline;
use rapier3d::dynamics::{
    CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet,
};
use rapier3d::geometry::{BroadPhase, NarrowPhase};
use rapier3d::math::Isometry;
use rapier3d::pipeline::PhysicsPipeline;
use rapier3d::{dynamics, geometry};
use std::marker::PhantomData;
use std::sync::RwLock;

#[derive(Component)]
pub struct S;
pub type NoUserData<'a> = &'a S;
//pub type NoUserData = S;
/// A plugin responsible for setting up a full Rapier physics simulation pipeline and resources.
///
/// This will automatically setup all the resources needed to run a Rapier physics simulation including:
/// - The physics pipeline.
/// - The integration parameters.
/// - The rigid-body, collider, and joint, sets.
/// - The gravity.
/// - The broad phase and narrow-phase.
/// - The event queue.
/// - Systems responsible for executing one physics timestep at each Bevy update stage.
pub struct RapierPhysicsPlugin<UserData>(PhantomData<UserData>);

impl<UserData> Default for RapierPhysicsPlugin<UserData> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
/*
pub fn reconcile_sys<UserData: 'static + WorldQuery>(
    mut commands: Commands,
    (time, mut sim_to_render_time): (ResMut<Time>, ResMut<SimulationToRenderTime>),
    (configuration, integration_parameters): (Res<RapierConfiguration>, Res<IntegrationParameters>),
    mut modifs_tracker: ResMut<ModificationTracker>,
    (
        mut pipeline,
        mut query_pipeline,
        mut islands,
        mut broad_phase,
        mut narrow_phase,
        mut ccd_solver,
        mut impulse_joints,
        mut multibody_joints,
        mut joints_entity_map,
    ): (
        ResMut<PhysicsPipeline>,
        ResMut<QueryPipeline>,
        ResMut<IslandManager>,
        ResMut<BroadPhase>,
        ResMut<NarrowPhase>,
        ResMut<CCDSolver>,
        ResMut<ImpulseJointSet>,
        ResMut<MultibodyJointSet>,
        ResMut<JointsEntityMap>,
    ),
    hooks: Res<PhysicsHooksWithQueryObject<UserData>>,
    (intersection_events, contact_events): (
        EventWriter<IntersectionEvent>,
        EventWriter<ContactEvent>,
    ),
    user_data: Query<UserData>,
    mut position_sync_query: Query<(Entity, &mut RigidBodyPositionSync)>,
    mut bodies_query: RigidBodyComponentsQuerySet,
    mut colliders_query: ColliderComponentsQuerySet,
    (removed_bodies, removed_colliders, removed_joints): (
        RemovedComponents<RigidBodyChangesComponent>,
        RemovedComponents<ColliderChangesComponent>,
        RemovedComponents<JointHandleComponent>,
    ),
    entities: &Entities,
    inp_buf: Res<InpBuf>,
    mut inp_his: ResMut<InputHistory>,
    mut 
) {
    println!(" buf: {:?}", inp_buf);
    let tick = inp_buf.tick;
    if inp_his.0.len() > 120 {
        for i in (inp_his.0.len() - 110)..(inp_his.0.len()) {
            println!(
                " his: {}, tick: {}",
                inp_his.0[i].translation, inp_his.0[i].tick
            );
        }
    }
    //
    // let intersection_events = &mut *intersection_events;
    let events = EventQueue {
        intersection_events: RwLock::new(intersection_events),
        contact_events: RwLock::new(contact_events),
    };

    let physics_hooks = PhysicsHooksWithQueryInstance {
        user_data: user_data,
        hooks: &*hooks.0,
    };

    for i in 0..inp_his.0.len() {
        //let t = intersection_events;
        let his = inp_his.0.pop().unwrap();
        if his.tick >= inp_buf.tick {
            

            {
                use std::mem::replace;
                /*

                */
                // modifs_tracker.detect_removals(removed_bodies, removed_colliders, removed_joints);
                modifs_tracker.detect_modifications(bodies_query.q1(), colliders_query.q1());

                let mut rigid_body_components_set = RigidBodyComponentsSet(bodies_query.q0());
                let mut collider_components_set = ColliderComponentsSet(colliders_query.q0());

                modifs_tracker.propagate_removals(
                    &entities,
                    &mut commands,
                    &mut islands,
                    &mut rigid_body_components_set,
                    &mut impulse_joints,
                    &mut multibody_joints,
                    &mut joints_entity_map,
                );
                islands.cleanup_removed_rigid_bodies(&mut rigid_body_components_set);

                match configuration.timestep_mode {
                    TimestepMode::InterpolatedTimestep => {
                        sim_to_render_time.diff += time.delta_seconds();

                        let sim_dt = integration_parameters.dt;
                        while sim_to_render_time.diff >= sim_dt {
                            if configuration.physics_pipeline_active {
                                // NOTE: in this comparison we do the same computations we
                                // will do for the next `while` iteration test, to make sure we
                                // don't get bit by potential float inaccuracy.
                                if sim_to_render_time.diff - sim_dt < sim_dt {
                                    // This is the last simulation step to be executed in the loop
                                    // Update the previous state transforms
                                    for (entity, mut position_sync) in
                                        position_sync_query.iter_mut()
                                    {
                                        if let RigidBodyPositionSync::Interpolated { prev_pos } =
                                            &mut *position_sync
                                        {
                                            let rb_pos: Option<&dynamics::RigidBodyPosition> =
                                                rigid_body_components_set.get(entity.handle());
                                            if let Some(rb_pos) = rb_pos {
                                                *prev_pos = Some(rb_pos.position);
                                            }
                                        }
                                    }
                                }
                                // let mut modified_bodies = modifs_tracker.modified_bodies.iter().map(|a| a.0).collect();
                                // let mut modified_colliders = modifs_tracker.modified_colliders.iter().map(|a|a.0).collect();
                                // let mut removed_colliders = modifs_tracker.removed_colliders.iter().map(|a|a.0).collect();
                                pipeline.step_generic(
                                    &configuration.gravity,
                                    &integration_parameters,
                                    &mut islands,
                                    &mut broad_phase,
                                    &mut narrow_phase,
                                    &mut rigid_body_components_set,
                                    &mut collider_components_set,
                                    &mut replace(&mut modifs_tracker.modified_bodies, vec![]),
                                    &mut replace(&mut modifs_tracker.modified_colliders, vec![]),
                                    &mut replace(&mut modifs_tracker.removed_colliders, vec![]),
                                    &mut impulse_joints,
                                    &mut multibody_joints,
                                    &mut ccd_solver,
                                    &physics_hooks,
                                    &events,
                                );
                                // modifs_tracker.modified_bodies = modified_bodies.iter().map(|a|RigidBodyHandle(*a)).collect();
                                // modifs_tracker.modified_colliders = modified_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                                // modifs_tracker.removed_colliders = removed_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                                modifs_tracker.clear_modified_and_removed();
                            }
                            sim_to_render_time.diff -= sim_dt;
                        }
                    }
                    TimestepMode::VariableTimestep | TimestepMode::FixedTimestep => {
                        if configuration.physics_pipeline_active {
                            let mut new_integration_parameters = *integration_parameters;

                            if configuration.timestep_mode == TimestepMode::VariableTimestep {
                                new_integration_parameters.dt =
                                    time.delta_seconds().min(integration_parameters.dt);
                            }
                            // let mut modified_bodies = modifs_tracker.modified_bodies.iter().map(|a| a.0).collect();
                            // let mut modified_colliders = modifs_tracker.modified_colliders.iter().map(|a|a.0).collect();
                            // let mut removed_colliders = modifs_tracker.removed_colliders.iter().map(|a|a.0).collect();
                            pipeline.step_generic(
                                &configuration.gravity,
                                &new_integration_parameters,
                                &mut islands,
                                &mut broad_phase,
                                &mut narrow_phase,
                                &mut rigid_body_components_set,
                                &mut collider_components_set,
                                &mut replace(&mut modifs_tracker.modified_bodies, vec![]),
                                &mut replace(&mut modifs_tracker.modified_colliders, vec![]),
                                &mut replace(&mut modifs_tracker.removed_colliders, vec![]),
                                &mut impulse_joints,
                                &mut multibody_joints,
                                &mut ccd_solver,
                                &physics_hooks,
                                &events,
                            );
                            // modifs_tracker.modified_bodies = modified_bodies.iter().map(|a|RigidBodyHandle(*a)).collect();
                            // modifs_tracker.modified_colliders = modified_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                            // modifs_tracker.removed_colliders = removed_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                            modifs_tracker.clear_modified_and_removed();
                        }
                    }
                }

                if configuration.query_pipeline_active {
                    query_pipeline.update_generic(
                        &islands,
                        &mut rigid_body_components_set,
                        &collider_components_set,
                    );
                }
            }
        }

        println!("");
        // Forwarding | Manually world step
    }
}

fn step_world<UserData: 'static + WorldQuery>(
    mut commands: &mut Commands,
    (time, mut sim_to_render_time): (ResMut<Time>, ResMut<SimulationToRenderTime>),
    (configuration, integration_parameters): (Res<RapierConfiguration>, Res<IntegrationParameters>),
    mut modifs_tracker: ResMut<ModificationTracker>,
    (
        mut pipeline,
        mut query_pipeline,
        mut islands,
        mut broad_phase,
        mut narrow_phase,
        mut ccd_solver,
        mut impulse_joints,
        mut multibody_joints,
        mut joints_entity_map,
    ): (
        ResMut<PhysicsPipeline>,
        ResMut<QueryPipeline>,
        ResMut<IslandManager>,
        ResMut<BroadPhase>,
        ResMut<NarrowPhase>,
        ResMut<CCDSolver>,
        ResMut<ImpulseJointSet>,
        ResMut<MultibodyJointSet>,
        ResMut<JointsEntityMap>,
    ),
    hooks: Res<PhysicsHooksWithQueryObject<UserData>>,
    (intersection_events, contact_events): (
        EventWriter<IntersectionEvent>,
        EventWriter<ContactEvent>,
    ),
    user_data: Query<UserData>,
    mut position_sync_query: Query<(Entity, &mut RigidBodyPositionSync)>,
    mut bodies_query: RigidBodyComponentsQuerySet,
    mut colliders_query: ColliderComponentsQuerySet,
    (removed_bodies, removed_colliders, removed_joints): (
        RemovedComponents<RigidBodyChangesComponent>,
        RemovedComponents<ColliderChangesComponent>,
        RemovedComponents<JointHandleComponent>,
    ),
    entities: &Entities,
) {
    use std::mem::replace;

    let events = EventQueue {
        intersection_events: RwLock::new(intersection_events),
        contact_events: RwLock::new(contact_events),
    };
    modifs_tracker.detect_removals(removed_bodies, removed_colliders, removed_joints);
    modifs_tracker.detect_modifications(bodies_query.q1(), colliders_query.q1());

    let mut rigid_body_components_set = RigidBodyComponentsSet(bodies_query.q0());
    let mut collider_components_set = ColliderComponentsSet(colliders_query.q0());

    modifs_tracker.propagate_removals(
        &entities,
        &mut commands,
        &mut islands,
        &mut rigid_body_components_set,
        &mut impulse_joints,
        &mut multibody_joints,
        &mut joints_entity_map,
    );
    islands.cleanup_removed_rigid_bodies(&mut rigid_body_components_set);

    let physics_hooks = PhysicsHooksWithQueryInstance {
        user_data,
        hooks: &*hooks.0,
    };

    match configuration.timestep_mode {
        TimestepMode::InterpolatedTimestep => {
            sim_to_render_time.diff += time.delta_seconds();

            let sim_dt = integration_parameters.dt;
            while sim_to_render_time.diff >= sim_dt {
                if configuration.physics_pipeline_active {
                    // NOTE: in this comparison we do the same computations we
                    // will do for the next `while` iteration test, to make sure we
                    // don't get bit by potential float inaccuracy.
                    if sim_to_render_time.diff - sim_dt < sim_dt {
                        // This is the last simulation step to be executed in the loop
                        // Update the previous state transforms
                        for (entity, mut position_sync) in position_sync_query.iter_mut() {
                            if let RigidBodyPositionSync::Interpolated { prev_pos } =
                                &mut *position_sync
                            {
                                let rb_pos: Option<&dynamics::RigidBodyPosition> =
                                    rigid_body_components_set.get(entity.handle());
                                if let Some(rb_pos) = rb_pos {
                                    *prev_pos = Some(rb_pos.position);
                                }
                            }
                        }
                    }
                    // let mut modified_bodies = modifs_tracker.modified_bodies.iter().map(|a| a.0).collect();
                    // let mut modified_colliders = modifs_tracker.modified_colliders.iter().map(|a|a.0).collect();
                    // let mut removed_colliders = modifs_tracker.removed_colliders.iter().map(|a|a.0).collect();
                    pipeline.step_generic(
                        &configuration.gravity,
                        &integration_parameters,
                        &mut islands,
                        &mut broad_phase,
                        &mut narrow_phase,
                        &mut rigid_body_components_set,
                        &mut collider_components_set,
                        &mut replace(&mut modifs_tracker.modified_bodies, vec![]),
                        &mut replace(&mut modifs_tracker.modified_colliders, vec![]),
                        &mut replace(&mut modifs_tracker.removed_colliders, vec![]),
                        &mut impulse_joints,
                        &mut multibody_joints,
                        &mut ccd_solver,
                        &physics_hooks,
                        &events,
                    );
                    // modifs_tracker.modified_bodies = modified_bodies.iter().map(|a|RigidBodyHandle(*a)).collect();
                    // modifs_tracker.modified_colliders = modified_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                    // modifs_tracker.removed_colliders = removed_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                    modifs_tracker.clear_modified_and_removed();
                }
                sim_to_render_time.diff -= sim_dt;
            }
        }
        TimestepMode::VariableTimestep | TimestepMode::FixedTimestep => {
            if configuration.physics_pipeline_active {
                let mut new_integration_parameters = *integration_parameters;

                if configuration.timestep_mode == TimestepMode::VariableTimestep {
                    new_integration_parameters.dt =
                        time.delta_seconds().min(integration_parameters.dt);
                }
                // let mut modified_bodies = modifs_tracker.modified_bodies.iter().map(|a| a.0).collect();
                // let mut modified_colliders = modifs_tracker.modified_colliders.iter().map(|a|a.0).collect();
                // let mut removed_colliders = modifs_tracker.removed_colliders.iter().map(|a|a.0).collect();
                pipeline.step_generic(
                    &configuration.gravity,
                    &new_integration_parameters,
                    &mut islands,
                    &mut broad_phase,
                    &mut narrow_phase,
                    &mut rigid_body_components_set,
                    &mut collider_components_set,
                    &mut replace(&mut modifs_tracker.modified_bodies, vec![]),
                    &mut replace(&mut modifs_tracker.modified_colliders, vec![]),
                    &mut replace(&mut modifs_tracker.removed_colliders, vec![]),
                    &mut impulse_joints,
                    &mut multibody_joints,
                    &mut ccd_solver,
                    &physics_hooks,
                    &events,
                );
                // modifs_tracker.modified_bodies = modified_bodies.iter().map(|a|RigidBodyHandle(*a)).collect();
                // modifs_tracker.modified_colliders = modified_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                // modifs_tracker.removed_colliders = removed_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                modifs_tracker.clear_modified_and_removed();
            }
        }
    }

    if configuration.query_pipeline_active {
        query_pipeline.update_generic(
            &islands,
            &mut rigid_body_components_set,
            &collider_components_set,
        );
    }
}
*/
pub fn step_world_system<UserData: 'static + WorldQuery>(
    mut commands: Commands,
    (time, mut sim_to_render_time): (Res<Time>, ResMut<SimulationToRenderTime>),
    (configuration, integration_parameters): (Res<RapierConfiguration>, Res<IntegrationParameters>),
    mut modifs_tracker: ResMut<ModificationTracker>,
    (
        mut pipeline,
        mut query_pipeline,
        mut islands,
        mut broad_phase,
        mut narrow_phase,
        mut ccd_solver,
        mut impulse_joints,
        mut multibody_joints,
        mut joints_entity_map,
    ): (
        ResMut<PhysicsPipeline>,
        ResMut<QueryPipeline>,
        ResMut<IslandManager>,
        ResMut<BroadPhase>,
        ResMut<NarrowPhase>,
        ResMut<CCDSolver>,
        ResMut<ImpulseJointSet>,
        ResMut<MultibodyJointSet>,
        ResMut<JointsEntityMap>,
    ),
    hooks: Res<PhysicsHooksWithQueryObject<UserData>>,
    (intersection_events, contact_events): (
        EventWriter<IntersectionEvent>,
        EventWriter<ContactEvent>,
    ),
    user_data: Query<UserData>,
    mut position_sync_query: Query<(Entity, &mut RigidBodyPositionSync)>,
    mut bodies_query: RigidBodyComponentsQuerySet,
    mut colliders_query: ColliderComponentsQuerySet,
    (removed_bodies, removed_colliders, removed_joints): (
        RemovedComponents<RigidBodyChangesComponent>,
        RemovedComponents<ColliderChangesComponent>,
        RemovedComponents<JointHandleComponent>,
    ),
    entities: &Entities,
) {
    use std::mem::replace;

    let events = EventQueue {
        intersection_events: RwLock::new(intersection_events),
        contact_events: RwLock::new(contact_events),
    };
    modifs_tracker.detect_removals(removed_bodies, removed_colliders, removed_joints);
    modifs_tracker.detect_modifications(bodies_query.q1(), colliders_query.q1());

    let mut rigid_body_components_set = RigidBodyComponentsSet(bodies_query.q0());
    let mut collider_components_set = ColliderComponentsSet(colliders_query.q0());

    modifs_tracker.propagate_removals(
        &entities,
        &mut commands,
        &mut islands,
        &mut rigid_body_components_set,
        &mut impulse_joints,
        &mut multibody_joints,
        &mut joints_entity_map,
    );
    islands.cleanup_removed_rigid_bodies(&mut rigid_body_components_set);

    let physics_hooks = PhysicsHooksWithQueryInstance {
        user_data,
        hooks: &*hooks.0,
    };

    match configuration.timestep_mode {
        TimestepMode::InterpolatedTimestep => {
            sim_to_render_time.diff += time.delta_seconds();

            let sim_dt = integration_parameters.dt;
            while sim_to_render_time.diff >= sim_dt {
                if configuration.physics_pipeline_active {
                    // NOTE: in this comparison we do the same computations we
                    // will do for the next `while` iteration test, to make sure we
                    // don't get bit by potential float inaccuracy.
                    if sim_to_render_time.diff - sim_dt < sim_dt {
                        // This is the last simulation step to be executed in the loop
                        // Update the previous state transforms
                        for (entity, mut position_sync) in position_sync_query.iter_mut() {
                            if let RigidBodyPositionSync::Interpolated { prev_pos } =
                                &mut *position_sync
                            {
                                let rb_pos: Option<&dynamics::RigidBodyPosition> =
                                    rigid_body_components_set.get(entity.handle());
                                if let Some(rb_pos) = rb_pos {
                                    *prev_pos = Some(rb_pos.position);
                                }
                            }
                        }
                    }
                    // let mut modified_bodies = modifs_tracker.modified_bodies.iter().map(|a| a.0).collect();
                    // let mut modified_colliders = modifs_tracker.modified_colliders.iter().map(|a|a.0).collect();
                    // let mut removed_colliders = modifs_tracker.removed_colliders.iter().map(|a|a.0).collect();
                    pipeline.step_generic(
                        &configuration.gravity,
                        &integration_parameters,
                        &mut islands,
                        &mut broad_phase,
                        &mut narrow_phase,
                        &mut rigid_body_components_set,
                        &mut collider_components_set,
                        &mut replace(&mut modifs_tracker.modified_bodies, vec![]),
                        &mut replace(&mut modifs_tracker.modified_colliders, vec![]),
                        &mut replace(&mut modifs_tracker.removed_colliders, vec![]),
                        &mut impulse_joints,
                        &mut multibody_joints,
                        &mut ccd_solver,
                        &physics_hooks,
                        &events,
                    );
                    // modifs_tracker.modified_bodies = modified_bodies.iter().map(|a|RigidBodyHandle(*a)).collect();
                    // modifs_tracker.modified_colliders = modified_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                    // modifs_tracker.removed_colliders = removed_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                    modifs_tracker.clear_modified_and_removed();
                }
                sim_to_render_time.diff -= sim_dt;
            }
        }
        TimestepMode::VariableTimestep | TimestepMode::FixedTimestep => {
            if configuration.physics_pipeline_active {
                let mut new_integration_parameters = *integration_parameters;

                if configuration.timestep_mode == TimestepMode::VariableTimestep {
                    new_integration_parameters.dt =
                        time.delta_seconds().min(integration_parameters.dt);
                }
                // let mut modified_bodies = modifs_tracker.modified_bodies.iter().map(|a| a.0).collect();
                // let mut modified_colliders = modifs_tracker.modified_colliders.iter().map(|a|a.0).collect();
                // let mut removed_colliders = modifs_tracker.removed_colliders.iter().map(|a|a.0).collect();
                pipeline.step_generic(
                    &configuration.gravity,
                    &new_integration_parameters,
                    &mut islands,
                    &mut broad_phase,
                    &mut narrow_phase,
                    &mut rigid_body_components_set,
                    &mut collider_components_set,
                    &mut replace(&mut modifs_tracker.modified_bodies, vec![]),
                    &mut replace(&mut modifs_tracker.modified_colliders, vec![]),
                    &mut replace(&mut modifs_tracker.removed_colliders, vec![]),
                    &mut impulse_joints,
                    &mut multibody_joints,
                    &mut ccd_solver,
                    &physics_hooks,
                    &events,
                );
                // modifs_tracker.modified_bodies = modified_bodies.iter().map(|a|RigidBodyHandle(*a)).collect();
                // modifs_tracker.modified_colliders = modified_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                // modifs_tracker.removed_colliders = removed_colliders.iter().map(|a|ColliderHandle(*a)).collect();
                modifs_tracker.clear_modified_and_removed();
            }
        }
    }

    if configuration.query_pipeline_active {
        query_pipeline.update_generic(
            &islands,
            &mut rigid_body_components_set,
            &collider_components_set,
        );
    }
}
