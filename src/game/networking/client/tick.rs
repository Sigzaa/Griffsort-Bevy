use bevy::{core::FixedTimestep,  prelude::*};
use crate::game::player_logic::client_controls::*;
use crate::game::components::*;
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

        // -> Add another stage
        .add_system_to_stage("tick", prepare_rollback.label("root").after("send")) // +

        .add_system_to_stage("tick", velocity_vector_sys.label("vel").after("root")) // +
        .add_system_to_stage("tick", predict_sys.label("predict").after("vel")) // +

        .add_system_to_stage("tick", update_tick.label("update_tick").after("predict")) // +
        // .add_system_to_stage(
        //     "tick",
        //     step_simulation::<()>
        //         .label("step")
        //         .after("update_tick"),
        // ) // and this also to backroll // +
        // <-
            .run();

    }
}
