use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::super::shared::systems::*;
use super::systems::*;
use crate::game::components::*;
use crate::game::player_logic::client_controls::*;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier3d::plugin::{systems::*, *};

// .add_system_set(
//     SystemSet::new()
//         .with_run_criteria(FixedTimestep::steps_per_second(TICKRATE))
//         //.with_system(crate::game::player_logic::shooting::shoot_system.label("addition").after("msg_collect"))
//         .with_system(pop_buffer.label("buffer").after("msg_collect"))
//         .with_system(crate::game::player_logic::client_controls::velocity_vector_sys.label("vector").after("buffer"))
//         .with_system(simulate_sys.label("sim").after("vector"))
//         .with_system(update_tick.label("tick").after("sim"))
//         //.with_system(step_world_system::<NoUserData>.label("world_step").after("tick"))
//         .with_system(send_sys.after("tick"))
// )

pub struct Tick;
impl Plugin for Tick {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PreUpdate,
            "tick",
            SystemStage::single_threaded()
                .with_run_criteria(FixedTimestep::steps_per_second(TICKRATE)),
        )
        .add_system_to_stage("tick", pop_buffer) // -
        .add_system_to_stage("tick", velocity_vector_sys.after(pop_buffer)) // +
        .add_system_to_stage("tick", simulate_sys.after(velocity_vector_sys)) // +
        .add_system_to_stage("tick", update_tick.label("update_tick").after(simulate_sys)) // +

        .add_system_set_to_stage(
            "tick",
            SystemSet::new()
                .label("step")
                .after("update_tick")
                .with_system(systems::init_async_colliders)
                .with_system(systems::apply_scale.after(systems::init_async_colliders))
                .with_system(systems::apply_collider_user_changes.after(systems::apply_scale))
                .with_system(
                    systems::apply_rigid_body_user_changes
                        .after(systems::apply_collider_user_changes),
                )
                .with_system(
                    systems::apply_joint_user_changes.after(systems::apply_rigid_body_user_changes),
                )
                .with_system(systems::init_rigid_bodies.after(systems::apply_joint_user_changes))
                .with_system(
                    systems::init_colliders
                        .after(systems::init_rigid_bodies)
                        .after(systems::init_async_colliders),
                )
                .with_system(systems::init_joints.after(systems::init_colliders))
                .with_system(systems::sync_removals.after(systems::init_joints))
                
                .with_system(systems::step_simulation::<()>.after(systems::sync_removals))
                .with_system(
                    systems::update_colliding_entities.after(systems::step_simulation::<()>),
                )
                .with_system(systems::writeback_rigid_bodies.after(update_colliding_entities)),
        )
        .add_system_to_stage("tick", send_sys.after("step")) // -
        .run();
    }
}
