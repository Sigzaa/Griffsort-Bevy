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
        //act.debug();
        for action in &act.just_pressed
        {
            
            match action
            {
                Action::Command(cmd) => enter_command(cmd, &mut command_entered),
                _ =>
                {}
            }
        }
    }
}

pub fn main_menu_script(command_entered: EventWriter<ConsoleCommandEntered>) {
    run_file("./config/main_menu.script", command_entered);
}

pub fn in_game_script(command_entered: EventWriter<ConsoleCommandEntered>) {
    run_file("./config/in_game.script", command_entered);
}

// Reading file
// Sending commands to console
fn run_file(filename: &'static str, mut command_entered: EventWriter<ConsoleCommandEntered>) {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename)
    {
        // Consumes the iterator, returns an (Optional) String
        for line in lines
        {
            if let Ok(cmd) = line
            {
                enter_command(&cmd, &mut command_entered);
            }
        }
    }
}

fn enter_command(cmd: &String, command_entered: &mut EventWriter<ConsoleCommandEntered>) {
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

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
