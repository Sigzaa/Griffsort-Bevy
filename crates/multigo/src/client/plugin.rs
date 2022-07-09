use super::{resources::*, systems::*};
use crate::shared::{resources::*, systems::*};
use bevy::prelude::*;
use bevy::core::FixedTimestep;
use std::{
    env,
    net::{SocketAddr, UdpSocket},
};

pub struct ClientPipeline;
impl Plugin for ClientPipeline {
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
            .add_system_to_stage(PhysNet, is_desync)
            .add_system_to_stage(PhysNet, send_input_history) // -
            .add_system_to_stage(PhysNet, update_tick.label("update_tick")) // +
            .run();
    }
}
