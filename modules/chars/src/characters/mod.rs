use bevy::{prelude::*};
use crate::game::components::{ *};

use soul::Soul;
mod soul;
mod extender;
mod root;

pub struct Characters;
impl Plugin for Characters {
    fn build(&self, app: &mut App) {
        app 
            .add_event::<ExtendCharacter>()
            .add_event::<SpawnCharacter>()

            .add_plugin(root::Root)
            .add_system(extender::extend)

            
            .add_plugin(Soul)
            //.add_plugin(DCover)
            ;
    }
}


