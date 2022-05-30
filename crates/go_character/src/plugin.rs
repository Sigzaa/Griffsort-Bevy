use crate::characters::*;
use crate::shared::{resources::*, systems::*};
use bevy::prelude::*;
use go_core::prelude::Character::*;
use heroes::*;
use std::marker::PhantomData;

pub struct Gocha;
impl Plugin for Gocha {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedId(None))
            .insert_resource(CharList(Vec::new()))
            .add_startup_system(_temp_setup)
            .add_plugin(CharPlugin::<DefaultChar>::new(DefaultChar))
            .add_plugin(CharPlugin::<Soul>::new(Soul));
    }
}

fn _temp_setup(
    mut spawn_request: EventWriter<SpawnChar>,
    mut selected: ResMut<SelectedId>,
    mut commands: Commands,
) {
    commands
        .spawn()
        .insert(CharacterCamera)
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.2, -0.5),
            perspective_projection: PerspectiveProjection {
                fov: 1.9,
                ..Default::default()
            },
            ..Default::default()
        });

    spawn_request.send(SpawnChar("Soul", 1, 0));
    spawn_request.send(SpawnChar("Soul", 1, 1));
    spawn_request.send(SpawnChar("Soul", 1, 2));
    spawn_request.send(SpawnChar("Soul", 1, 3));
    spawn_request.send(SpawnChar("Soul", 1, 4));
    spawn_request.send(SpawnChar("Soul", 1, 5)); // Spawning Soul in team 1 with id 0
    selected.0 = Some(5);
}
