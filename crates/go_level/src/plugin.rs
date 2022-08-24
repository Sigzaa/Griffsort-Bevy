use super::{systems::*};
use bevy_atmosphere::prelude::*;
use bevy::prelude::*;
use corgee::*;
pub struct Level;
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(AtmospherePlugin)
            .add_state(Map::None)
            .add_startup_system(load_map)
            .add_startup_system(load_custom_models);
    }
}   
