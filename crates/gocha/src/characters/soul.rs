use heroes::*;
use bevy::prelude::*;
use networking::prelude::*;
use core::prelude::Character::*;
#[derive(Copy, Clone)]
pub struct Soul;

impl Character<Soul> for Soul { }

impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app
        //.add_system_set_to_stage()
        .add_event::<SpawnCharacterRequest>()
        .add_system(Soul::spawn::<1>)
        
        ;
    }
}

impl Soul {
    fn spawn<const LAYER_ID: i32>(
        mut spawn_request: EventReader<SpawnCharacterRequest>,
    ){
        
    }
}




