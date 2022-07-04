use super::{resources::*, systems::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use corgee::*;
use bevy_atmosphere::*;
use bevy_transform_gizmo::*;
pub struct Level;
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app.add_state(Map::None)
            .add_startup_system(load_map)
            // .add_plugins(bevy_mod_picking::DefaultPickingPlugins)
            // .add_plugin(bevy_transform_gizmo::TransformGizmoPlugin::new(
            //     Quat::from_rotation_y(-0.2), // Align the gizmo to a different coordinate system.
            // ))
            .insert_resource(bevy_atmosphere::AtmosphereMat::default())
            .add_plugin(bevy_atmosphere::AtmospherePlugin {
                dynamic: false, // Set to false since we aren't changing the sky's appearance
                sky_radius: 50.0,
            });
    }
}
