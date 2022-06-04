use crate::goinputs::systems::*;
use bevy::prelude::*;

pub struct Core;
impl Plugin for Core {
    fn build(&self, app: &mut App) {
        app
        .add_system(collect_inputs)
        .add_system(camera_motion)
        ;
    }
}
