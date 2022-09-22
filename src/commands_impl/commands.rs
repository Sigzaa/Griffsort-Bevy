use bevy::prelude::Commands;
use bevy_console::*;
use gs_states::{NextState, GameState};

// Example command
#[derive(ConsoleCommand)]
#[console_command(name = "run")]
pub struct RunCommand {
    /// Some message
    script_path: String,
}

pub fn run_command(mut log: ConsoleCommand<RunCommand>) {
    if let Some(RunCommand { script_path }) = log.take() {
        // handle command
    }
}

// Example command
#[derive(ConsoleCommand)]
#[console_command(name = "connect")]
pub struct ConnectCommand {
    /// Ip address
    ip: String,
}

pub fn connect_command(mut log: ConsoleCommand<ConnectCommand>, mut commands: Commands) {
    if let Some(ConnectCommand { ip }) = log.take() {
        commands.insert_resource(NextState(GameState::InGame));
        // handle command
    }
}

