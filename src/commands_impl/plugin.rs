use super::{
    commands::*,
    conf_command::{conf_command, ConfCommand},
    systems::{in_game_script, main_menu_script, run_binded_commands},
};
use bevy::prelude::{Plugin, State};
use gs_inspector::AddConsoleCommand;
use gs_states::{AppLooplessStateExt, GameState, IntoConditionalSystem};
pub struct ConsoleCommands;

impl Plugin for ConsoleCommands {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_console_command::<RunCommand, _, _>(run_command)
            .add_console_command::<ConnectCommand, _, _>(connect_command)
            .add_console_command::<ConfCommand, _, _>(conf_command)
            .add_console_command::<SpawnCommand, _, _>(spawn_command)
            .add_console_command::<MatchCommand, _, _>(match_command)
            .add_console_command::<WatchmeCommand, _, _>(watchme_command)
            .add_system(watchme_look_at)
            .add_system(run_binded_commands)
            .add_enter_system(GameState::MainMenu, main_menu_script)
            .add_enter_system(GameState::InGame, in_game_script);
    }
}
