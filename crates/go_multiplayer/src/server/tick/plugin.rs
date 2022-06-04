use bevy::prelude::*;


use crate::shared::{systems::*, resources::*};
use super::systems::*;
use bevy::{core::FixedTimestep, prelude::*};
use crate::shared::resources::TICKRATE;


pub struct Tick;
impl Plugin for Tick {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::PreUpdate,
            NetStage,
            SystemStage::single_threaded()
                .with_run_criteria(FixedTimestep::steps_per_second(TICKRATE as f64)),
        )
        .add_system_to_stage(NetStage, pop_buffer.label("root")) // -

        .add_system_to_stage(NetStage, update_tick.label("update_tick")) // +

        // .add_system_set_to_stage(
        //     NetStage,
        //     SystemSet::new()
        //         .label("step")
        //         .after("update_tick")
        //         .with_system(systems::init_async_colliders)
        //         .with_system(systems::apply_scale.after(systems::init_async_colliders))
        //         .with_system(systems::apply_collider_user_changes.after(systems::apply_scale))
        //         .with_system(
        //             systems::apply_rigid_body_user_changes
        //                 .after(systems::apply_collider_user_changes),
        //         )
        //         .with_system(
        //             systems::apply_joint_user_changes.after(systems::apply_rigid_body_user_changes),
        //         )
        //         .with_system(systems::init_rigid_bodies.after(systems::apply_joint_user_changes))
        //         .with_system(
        //             systems::init_colliders
        //                 .after(systems::init_rigid_bodies)
        //                 .after(systems::init_async_colliders),
        //         )
        //         .with_system(systems::init_joints.after(systems::init_colliders))
        //         .with_system(systems::sync_removals.after(systems::init_joints))
                
        //         .with_system(systems::step_simulation::<()>.after(systems::sync_removals))
        //         .with_system(
        //             systems::update_colliding_entities.after(systems::step_simulation::<()>),
        //         )
        //         .with_system(systems::writeback_rigid_bodies.after(update_colliding_entities)),
        // )
        //.add_system_to_stage(NetStage, send_sys.after("step")) // -
        .run();
    }
}