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

        .add_system_set(
            ConditionSet::new()
            .with_system(insert_other)
            .with_system(shoot)
            .with_system(walk::<YuiShang, ShangConfig>)
            .with_system(look::<YuiShang>.run_if(cursor_showed))
            .with_system(camera_shake::<YuiShang, ShangConfig>)
            .into());
    }
}

pub fn insert_other(
    mut commands: Commands,
    query: Query<Entity, Added<YuiShang>>,
) {
    for entity in query.iter() {

        commands
        .entity(entity)
        .insert(ShootCD(CDProps::new(14)));
    }
}

