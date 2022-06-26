use super::resys::*;
use bevy::prelude::*;
use corgee::GameState;

pub struct UI;
impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(startup_menu))
            .add_system_set(SystemSet::on_update(GameState::MainMenu).with_system(label_update).with_system(button_system))
            .add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(cleanup));
    }
}
