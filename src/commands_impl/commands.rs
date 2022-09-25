use actions::Actions;
use bevy::{prelude::*, transform::TransformBundle};
use bevy_console::{*};
use bevy_console_parser::parse_console_command;
use gs_states::{NextState, GameState};
use heroes::{*};

use crate::{Action, characters_impl::{Jacqueline, SpawnHeroEv}};

//use crate::heroes_mapping::spawn_hero;

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

#[derive(ConsoleCommand)]
#[console_command(name = "match")]
pub struct MatchCommand {
    /// Some message
    script_path: Option<String>,
}

pub fn match_command(
    mut log: ConsoleCommand<MatchCommand>,
    query: Query<(Entity, &Id, &Team), With<Hero>>
) {
    if let Some(MatchCommand { script_path }) = log.take() {
        for (ent, id, team) in &query{
            reply!(log, "ECS Entity: {:?}, GS`s Id: {}, Team: {:?}", ent, id.0, team);
        }

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

#[derive(ConsoleCommand)]
#[console_command(name = "spawn")]
pub struct SpawnCommand {
    /// Hero Name
    name: String,
    /// Hero Team (Dark | Light) (1 | 0)
    team: String,

    id: Option<i32>
}

pub fn spawn_command(
    mut log: ConsoleCommand<SpawnCommand>, 
    query: Query<&Transform, With<Selected>>,
    mut spawner: EventWriter<SpawnHeroEv>,
    mut commands: Commands
) {
    if let Some(SpawnCommand { name, team , id}) = log.take() {
        for transform in &query
        {
            let team = match &team[..]{
                "light" | "0" => Team::Light,
                "dark" | "1" => Team::Dark,
                _=> Team::Light
            };

            spawner.send(SpawnHeroEv::new(name.clone(), team, transform.translation + transform.forward() * 3.5 + Vec3::new(0., 2., 0.)));
            log.ok();
        }
    }
}

