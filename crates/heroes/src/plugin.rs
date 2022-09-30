use crate::{hud::*, id_manager::IdManager, systems::sync_selected};

use super::controller::resources::*;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use bevy_prototype_debug_lines::{DebugLinesPlugin};
use bevy_rapier3d::prelude::*;
use synx::Synx;

pub struct Heroes;
impl Plugin for Heroes {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)

            // Physics
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            //.add_plugin(RapierDebugRenderPlugin::default())

            // Syncing HeroesConfig resource with filesystem
            .add_plugin(Synx::<HeroesConfig>::new("./config/heroes.ron"))

            // Adding Egui controller on top of it's resource
            .add_plugin(InspectorPlugin::<HeroesConfig>::new())

            // Plugin for drawing lines in 3d 
            .add_plugin(DebugLinesPlugin::with_depth_test(true))

            // Global API for changing selected Hero
            .insert_resource(SelectedId(None))

            // For allocating, removing and etc. ids
            .insert_resource(IdManager::new())

            // Syncing SelectedId res with Selected and SelectedCam components
            .add_system(sync_selected)

            // Egui hud
            .add_system(hp_bar);
    }
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

// fn sync_heroes_config(
//     mut rapier_debug: ResMut<DebugRenderContext>,
//     config: Res<HeroesConfig>
// ){
//     if config.is_changed(){
//         rapier_debug.enabled = config.debug_rapier;
//     }
// }
