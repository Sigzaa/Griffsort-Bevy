use super::*;
use bevy::prelude::*;
use go_character::*;
use corgee::character::*;

impl Plugin for Zero {
    fn build(&self, _app: &mut App) {}
}

impl Character<Zero> for Zero {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter() {
            if spawn_request.0 == "Zero" {
                commands
                .spawn()
                .insert(Zero)
                .insert_bundle(Config {
                    ..Default::default()
                })
                .insert_bundle(States {
                    id: Id(spawn_request.2),
                    ..Default::default()
                });
            }
        }
    }
}
