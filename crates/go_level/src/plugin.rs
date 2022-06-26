use super::{resources::*, systems::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use corgee::*;
use bevy_atmosphere::*;
pub struct Level;
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app.add_state(Map::None)
            .add_startup_system(load_map)
            .insert_resource(bevy_atmosphere::AtmosphereMat::default())
            .add_plugin(bevy_atmosphere::AtmospherePlugin {
                dynamic: false, // Set to false since we aren't changing the sky's appearance
                sky_radius: 50.0,
            });
    }
}
