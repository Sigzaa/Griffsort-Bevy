use super::*;
use bevy::prelude::*;
use heroes::*;

impl Plugin for Zero {
    fn build(&self, app: &mut App) {
        // app
        // .add_system(look::<Zero>)
        // .add_system(walk::<Zero>);
    }
}

impl Character<Zero> for Zero {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter()
        {
            if spawn_request.0 == "Zero"
            {
                commands
                    .spawn()
                    .insert(Zero)
                    .insert_bundle(States::new_with_id(spawn_request.2));
            }
        }
    }
}
