use super::controller::resources::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use bevy_rapier3d::prelude::*;

pub struct CharController;
impl Plugin for CharController {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(DebugLinesPlugin::default())
            .insert_resource(SelectedId(None))
            .insert_resource(HeroesConfig::default());
    }
}
fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
