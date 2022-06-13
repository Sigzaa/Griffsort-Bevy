use bevy::prelude::*;
use super::{systems::*, resources::*};
use bevy_rapier3d::prelude::*;
use go_core::*;
pub struct Level;
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app
        .add_state(Map::None)
        .add_startup_system(load_map)    
        ;
    }
}
