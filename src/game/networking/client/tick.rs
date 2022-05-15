use bevy::{core::FixedTimestep,  prelude::*};
use crate::game::player_logic::client_controls::*;
use crate::game::components::*;
use bevy_rapier3d::plugin::{systems::*, *};
use super::super::shared::{systems::*, };
use super::systems::*;

pub struct Tick;
impl Plugin for Tick {
    fn build(&self, app: &mut App) {
       

        app
        .add_stage_after(
            CoreStage::PreUpdate,
            "tick",
            SystemStage::single_threaded()
                .with_run_criteria(FixedTimestep::steps_per_second(TICKRATE)),
        )
        .add_system(collect_inputs_sys.label("collect_inputs"))

        .add_system_to_stage(
            "tick",
            check_for_desync_sys.with_run_criteria(tick_more_then_zero), // -
        )
        .add_system_to_stage("tick", send_message.label("send").after("collect_inputs")) // -
        .add_system_to_stage("tick", prepare_rollback.label("root").after("send")) // +
        // -> Add another stage
        .add_system_to_stage("tick", save_snap.after(prepare_rollback)) // +

        .add_system_to_stage("tick", velocity_vector_sys.after(save_snap)) // +
        .add_system_to_stage("tick", predict_sys.label("predict").after("vel")) // +

        .add_system_to_stage("tick", update_tick.label("update_tick").after("predict")) // +
        // .add_system_to_stage(
        //     "tick",
        //     step_simulation::<()>.label("step")
        //         .after(update_tick),
        // ) // and this also to backroll // +
        // <-
        .add_system_set_to_stage(
            "tick",
            SystemSet::new().label("back").after("update_tick")
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

        )
        .add_system_set_to_stage(
            "tick",
            SystemSet::new().label("step").after("back")
            .with_system(systems::step_simulation::<()>)

        )
        .add_system_set_to_stage(
            "tick",
            SystemSet::new().label("write_back").after("step")
                .with_system(systems::update_colliding_entities)
                .with_system(systems::writeback_rigid_bodies),

        )


        .run();

    }
}
