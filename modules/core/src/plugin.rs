use bevy::{ prelude::*};
use crate::goinputs::systems::*;


pub struct GoInputs;
impl Plugin for GoInputs {
    fn build(&self, app: &mut App) {
        app
            .add_system(collect_inputs)

        ;
    }
}
