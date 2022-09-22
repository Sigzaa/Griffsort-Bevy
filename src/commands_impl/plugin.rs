use bevy::prelude::Plugin;
use super::{commands::*, conf_command::{ConfCommand, conf_command}};
use gs_inspector::AddConsoleCommand;

pub struct ConsoleCommands;

impl Plugin for ConsoleCommands{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
        .add_console_command::<RunCommand, _, _>(run_command)
        .add_console_command::<ConnectCommand, _, _>(connect_command)
        .add_console_command::<ConfCommand, _, _>(conf_command)
        ;
    }
}