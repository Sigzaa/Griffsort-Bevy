use crate::goinputs::{systems::*, resources::*, gamepad::*};
use crate::goconfig::Config;
use bevy::prelude::*;
use super::states::GameState;

pub struct Corgee;
impl Plugin for Corgee {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(Config)
        .add_state(GameState::MainMenu)
        .insert_resource(Sensitivity(0.1))

        .add_system(gamepad_connections)
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                .with_system(camera_motion)
                .with_system(collect_inputs)
        )
        //.add_system(gamepad_input.with_run_criteria(run_if_gamepad_connected).with_run_criteria(if_not_server))
        
        ;
    }
}
