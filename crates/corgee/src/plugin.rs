use crate::goinputs::{systems::*, resources::*, gamepad::*};
use crate::goconfig::Config;
use bevy::prelude::*;
use super::states::GameState;
use super::shared::{systems::*, resources::*};

pub struct Corgee;
impl Plugin for Corgee {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(Config)
        .add_state(GameState::MainMenu)
        .insert_resource(Sensitivity(0.5))
        .insert_resource(CursorIsHided(false))
        .add_system(alt_switch_cursor)
        .add_system(handle_cursor)
        .add_system(gamepad_connections)
        .add_system_set(
            SystemSet::on_update(GameState::InGame)
                //.with_system(camera_motion)
                .with_system(collect_inputs)
        )
        .add_system_set(
            SystemSet::on_resume(GameState::InGame)
                //.with_system(camera_motion)
                .with_system(hide_cursor)
        )
        .add_system_set(
            SystemSet::on_enter(GameState::InGame)
                //.with_system(camera_motion)
                .with_system(hide_cursor)
        )
        .add_system_set(
            SystemSet::on_pause(GameState::InGame)
                //.with_system(null_inputs)
                .with_system(unhide_cursor)
        )
        //.add_system(gamepad_input.with_run_criteria(run_if_gamepad_connected).with_run_criteria(if_not_server))
        
        ;
    }
}
