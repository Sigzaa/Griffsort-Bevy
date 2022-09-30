
use bevy::{prelude::*, render::camera::Projection};
use bevy_console::*;

use gs_states::{GameState, NextState};
use heroes::*;

use crate::{
    characters_impl::{SpawnHeroEv},
};

//use crate::heroes_mapping::spawn_hero;

// Example command
#[derive(ConsoleCommand)]
#[console_command(name = "run")]
pub struct RunCommand {
    /// Some message
    script_path: String,
}

pub fn run_command(mut log: ConsoleCommand<RunCommand>) {
    if let Some(RunCommand { script_path: _ }) = log.take()
    {
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
    query: Query<(Entity, &Id, &Team), With<Hero>>,
) {
    if let Some(MatchCommand { script_path: _ }) = log.take()
    {
        for (ent, id, team) in &query
        {
            reply!(
                log,
                "ECS Entity: {:?}, GS`s Id: {}, Team: {:?}",
                ent,
                id.0,
                team
            );
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
    if let Some(ConnectCommand { ip: _ }) = log.take()
    {
        commands.insert_resource(NextState(GameState::InGame));
        // handle command
    }
}

// Example command
#[derive(ConsoleCommand)]
#[console_command(name = "watchme")]
pub struct WatchmeCommand {
    /// Ip address
    x: Option<String>,
    y: Option<String>,
    z: Option<String>,
}

#[derive(Component)]
pub struct WatchmeCam;

pub fn watchme_command(
    mut log: ConsoleCommand<WatchmeCommand>,
    mut commands: Commands,
    watchme_cam: Query<Entity, With<WatchmeCam>>,
    sel: Query<&Transform, With<Selected>>,
) {
    if let Some(WatchmeCommand { x, y, z }) = log.take()
    {
        for ent in &watchme_cam
        {
            commands.entity(ent).despawn();
        }

        match (x, y, z)
        {
            (Some(x), Some(y), Some(z)) =>
            {
                let transform = sel.single();

                let x = x.parse::<f32>().unwrap_or(transform.translation[0]);
                let y = y.parse::<f32>().unwrap_or(transform.translation[1]);
                let z = z.parse::<f32>().unwrap_or(transform.translation[2] + 5.);

                commands
                    .spawn_bundle(Camera3dBundle {
                        projection: Projection::Perspective(PerspectiveProjection {
                            fov: 1.4, // a float of your fov in radians,
                            ..default()
                        }),
                        camera: Camera {
                            is_active: true,
                            priority: 2,
                            ..Default::default()
                        },
                        transform: Transform::from_translation(
                            transform.translation + Vec3::new(x, y, z),
                        ),

                        ..Default::default()
                    })
                    .insert(WatchmeCam);
            }
            _ =>
            {}
        }
        // handle command
    }
}

pub fn watchme_look_at(
    mut watchme_cam: Query<&mut Transform, (With<WatchmeCam>, Without<Selected>)>,
    sel: Query<&Transform, With<Selected>>,
) {
    let transform = sel.get_single();

    if let Ok(transform) = transform
    {
        for mut cam_transform in &mut watchme_cam
        {
            *cam_transform = cam_transform.looking_at(transform.translation, Vec3::Y);
        }
    }
}

#[derive(ConsoleCommand)]
#[console_command(name = "spawn")]
pub struct SpawnCommand {
    /// Hero Name
    name: String,
    /// Hero Team (Dark | Light) (1 | 0)
    team: String,

    id: Option<i32>,
}

pub fn spawn_command(
    mut log: ConsoleCommand<SpawnCommand>,
    query: Query<&Transform, With<Selected>>,
    mut spawner: EventWriter<SpawnHeroEv>,
) {
    if let Some(SpawnCommand { name, team, id: _ }) = log.take()
    {
        for transform in &query
        {
            let team = match &team[..]
            {
                "light" | "0" => Team::Light,
                "dark" | "1" => Team::Dark,
                _ => Team::Light,
            };

            spawner.send(SpawnHeroEv::new(
                name.clone(),
                team,
                transform.translation + transform.forward() * 3.5 + Vec3::new(0., 2., 0.),
            ));
            log.ok();
        }
    }
}
