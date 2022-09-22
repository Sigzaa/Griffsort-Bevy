mod abils;
pub mod resources;

use crate::Action;

use self::resources::{ShootCD, ShangConfig};

use super::{YuiShang, default::*};
use actions::Actions;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use gs_states::{ConditionSet, IntoConditionalSystem, cursor_showed};
use heroes::*;
use abils::*;
use synx::Synx;


impl Plugin for YuiShang{
    fn build(&self, app: &mut App) {
        app
        //.insert_resource(ShangConfig::default())
        .add_plugin(InspectorPlugin::<ShangConfig>::new())
        .add_plugin(Synx::<ShangConfig>::new("./config/yui_shang.ron"))
        .add_system_set(
            ConditionSet::new()
            .with_system(shoot)
            .with_system(walk::<YuiShang, ShangConfig>)
            .with_system(look::<YuiShang>.run_if(cursor_showed))
            .with_system(camera_shake::<YuiShang, ShangConfig>)
            .into());
    }
}

impl Character<YuiShang> for YuiShang{
    fn spawn(mut spawn_request: EventReader<heroes::SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter()
        {
            if spawn_request.0 == "Yui Shang"
            {
                commands
                    .spawn()
                    .insert(YuiShang)
                    .insert(CollisionGroups::new(0b01, 0b110))
                    .insert(Actions::<Action>::new())
                    .insert(PointingOn(Vec::new()))
                    .insert(ShootCD(CDProps::new(14)))
                    .insert_bundle(States {
                        id: Id(spawn_request.2),
                        team: Team::Dark,
                        ..Default::default()
                    });
            }
        }
    }
}