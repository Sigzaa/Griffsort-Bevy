use crate::shared::{resources::*, systems::*};
use bevy::prelude::*;
use corgee::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};


pub struct CharController;
impl Plugin for CharController {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup)

            //.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(DebugLinesPlugin::default())
            .insert_resource(SelectedId(None))
            .insert_resource(ShowRay(true))
            .insert_resource(CharList(Vec::new()));
    }
}
fn setup(mut commands: Commands){
    commands.spawn_bundle(Camera2dBundle::default());
}