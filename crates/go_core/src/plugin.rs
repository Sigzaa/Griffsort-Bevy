use crate::goinputs::{systems::*, resources::*, gamepad::*};
use bevy::prelude::*;

pub struct Core;
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(Sensitivity(0.1))
        .add_system(gamepad_connections)
        .add_system(camera_motion)
        .add_system(collect_inputs.with_run_criteria(run_if_gamepad_disconnected))
        .add_system(gamepad_input.with_run_criteria(run_if_gamepad_connected))
        
        ;
    }
}
