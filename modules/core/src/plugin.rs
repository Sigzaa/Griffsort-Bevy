use bevy::{ prelude::*};
use crate::goinputs::systems::*;


pub struct Core;
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app
            .add_system(collect_inputs)

        ;
    }
}
