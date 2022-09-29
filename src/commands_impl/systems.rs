use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

use actions::Actions;
use bevy::prelude::{EventWriter, *};
use bevy_console::{ConsoleCommandEntered, ValueRawOwned};
use bevy_console_parser::parse_console_command;
use heroes::Selected;

use crate::Action;

pub fn run_binded_commands(
    query: Query<&Actions<Action>, With<Selected>>,
    mut command_entered: EventWriter<ConsoleCommandEntered>,
) {
    for act in &query
    {
        for action in &act.just_pressed
        {
            match action
            {
                Action::Command(cmd) => match parse_console_command(&cmd)
                {
                    Ok(cmd) =>
                    {
                        let command = ConsoleCommandEntered {
                            command: cmd.command.to_string(),
                            args: cmd.args.into_iter().map(ValueRawOwned::from).collect(),
                        };

                        command_entered.send(command);
                    }
                    Err(_) =>
                    {}
                },
                _ =>
                {}
            }
        }
    }
}

pub fn main_menu_script(mut command_entered: EventWriter<ConsoleCommandEntered>) {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./config/main_menu.script")
    {
        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(cmd) = line
            {
                match parse_console_command(&cmd)
                {
                    Ok(cmd) =>
                    {
                        let command = ConsoleCommandEntered {
                            command: cmd.command.to_string(),
                            args: cmd.args.into_iter().map(ValueRawOwned::from).collect(),
                        };

                        command_entered.send(command);
                    }
                    Err(_) =>
                    {}
                }
            }
        }
    }
}

pub fn in_game_script(mut command_entered: EventWriter<ConsoleCommandEntered>) {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./config/in_game.script")
    {
        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(cmd) = line
            {
                match parse_console_command(&cmd)
                {
                    Ok(cmd) =>
                    {
                        let command = ConsoleCommandEntered {
                            command: cmd.command.to_string(),
                            args: cmd.args.into_iter().map(ValueRawOwned::from).collect(),
                        };

                        command_entered.send(command);
                    }
                    Err(_) =>
                    {}
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
