mod abils;
pub mod resources;

use crate::Action;

use self::resources::{ShangConfig, ShootCD};

use super::{default::*, YuiShang};
use abils::*;
use actions::Actions;
use bevy::prelude::*;

use gs_states::{cursor_showed, ConditionSet, IntoConditionalSystem};
use heroes::*;

impl Plugin for YuiShang {
    fn build(&self, app: &mut App) {
        app
            //.insert_resource(ShangConfig::default())
            .add_system_set(
                ConditionSet::new()
                    .with_system(insert_other)
                    .with_system(shoot)
                    .with_system(walk::<YuiShang, ShangConfig>)
                    .with_system(look::<YuiShang>.run_if(cursor_showed))
                    .with_system(is_grounded::<YuiShang, ShangConfig>)
                    .with_system(jump::<YuiShang, ShangConfig>)
                    .with_system(camera_shake::<YuiShang, ShangConfig>)
                    .into(),
            );
    }
}

pub fn insert_other(mut commands: Commands, query: Query<Entity, Added<YuiShang>>) {
    for entity in query.iter()
    {
        commands.entity(entity).insert(ShootCD(CDProps::new(14)));
    }
}
