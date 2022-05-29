use bevy::{ prelude::*};
use crate::shared::{systems::*, resources::*};
use crate::characters::*;
use core::prelude::Character::*;
use heroes::*;
use std::marker::PhantomData;

pub struct Gocha;
impl Plugin for Gocha {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedId(None))
            .insert_resource(CharList(Vec::new()))
            .add_plugin(CharPlugin::<DefaultChar>::new(DefaultChar))
            .add_plugin(CharPlugin::<Soul>::new(Soul))
            .add_startup_system(_temp_setup)
            
        ;
    }
}

fn _temp_setup(
    mut spawn_request: EventWriter<SpawnCharacterRequest>,
){
    spawn_request.send(SpawnCharacterRequest(1 , 1)); // Spawning Soul in team 1.
}
