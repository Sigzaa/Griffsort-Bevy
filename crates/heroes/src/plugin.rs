use crate::{hud::*, systems::sync_selected, id_manager::IdManager};

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
            .insert_resource(HeroesConfig::default())
            .insert_resource(IdManager::new())
            .add_system(sync_selected)
            .add_system(hp_bar)
            //.add_system(sync_heroes_config)
            ;
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
