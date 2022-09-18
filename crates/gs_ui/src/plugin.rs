use super::resys::*;
use bevy::prelude::*;
use gs_states::{AppLooplessStateExt, GameState, IntoConditionalSystem};

pub struct UI;
impl Plugin for UI {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, startup_menu)
            .add_system(label_update.run_in_state(GameState::MainMenu))
            .add_system(button_system.run_in_state(GameState::MainMenu))
            .add_exit_system(GameState::MainMenu, cleanup);
    }
}
