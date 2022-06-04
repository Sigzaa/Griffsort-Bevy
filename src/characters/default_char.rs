use go_character::*;
use bevy::prelude::*;
use super::*;




impl Plugin for DefaultChar {
    fn build(&self, app: &mut App) {
        app
        //.add_system_to_stage(DefaultChar::movement)
        ;
    }
}

impl Character<DefaultChar> for DefaultChar {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter() {
            if spawn_request.0 == "Default" {
                commands.spawn().insert(Soul);
            }
        }
    }
}
