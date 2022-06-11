use crate::shared::{resources::*, systems::*};
use bevy::prelude::*;
use go_core::Character::*;
use bevy_rapier3d::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};


pub struct CharController;
impl Plugin for CharController {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(DebugLinesPlugin::default())
            .insert_resource(SelectedId(None))
            .insert_resource(ShowRay(true))
            .insert_resource(CharList(Vec::new()));
    }
}