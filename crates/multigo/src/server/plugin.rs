use super::{systems::*, resources::*};
use crate::shared::{resources::*, systems::*};
use bevy::{core::FixedTimestep, prelude::*, app::ScheduleRunnerSettings};
use std::{time::Duration, env, str, net::{ UdpSocket }};

#[derive(Default)]
pub struct IsStarted(pub bool);

pub struct ServerPipeline;
impl Plugin for ServerPipeline {
    fn build(&self, app: &mut App) {
        app 

            .add_system(connection_handler)
            .add_system(receive_handler)

            .add_stage_after(
                CoreStage::PreUpdate,
                PhysNet,
                SystemStage::single_threaded()
                    .with_run_criteria(FixedTimestep::steps_per_second(CONST_TICKRATE)),
            )
            .add_system_to_stage(PhysNet, pop_buffer.label("root")) // -
    
            .add_system_to_stage(PhysNet, update_tick.label("update_tick")) // +
            //.add_system(send_message)

            .run();
    }
}
