use heroes::*;
use bevy::prelude::*;
use networking::prelude::*;
#[derive(Copy, Clone)]
pub struct Soul;
impl Character for Soul { }

impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app
        //.add_system_set_to_stage()
        ;


    }
}

impl Soul {

    fn shoot(){

    }
}




