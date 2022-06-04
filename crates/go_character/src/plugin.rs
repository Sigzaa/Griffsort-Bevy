
use crate::shared::{resources::*, systems::*};
use bevy::prelude::*;
use go_core::prelude::Character::*;
use bevy_rapier3d::prelude::*;

pub struct CharController;
impl Plugin for CharController {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_plugin(RapierDebugRenderPlugin::default())
            .insert_resource(SelectedId(None))
            .insert_resource(CharList(Vec::new()));
    }
}
