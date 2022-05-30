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
            .add_startup_system(_temp_setup)
            .add_plugin(CharPlugin::<DefaultChar>::new(DefaultChar))
            .add_plugin(CharPlugin::<Soul>::new(Soul))
            
            
        ;
    }
}

fn _temp_setup(
    mut spawn_request: EventWriter<SpawnChar>,
    mut selected: ResMut<SelectedId>,
    
){
    spawn_request.send(SpawnChar("Soul" , 1, 0)); // Spawning Soul in team 1.
    selected.0 = Some(0);
}
