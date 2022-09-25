pub mod resources;
mod abilities;

use actions::Actions;
use bevy_inspector_egui::{InspectorPlugin, widgets::InspectorQuerySingle};
use gs_states::{ConditionSet, IntoConditionalSystem, cursor_showed};
use synx::Synx;
use abilities::*;

use crate::Action;

use self::resources::{JacquelineConfig, QueryT};

use super::{*, default::*};

impl Plugin for Jacqueline{
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                ConditionSet::new()
                    .with_system(insert_other)
                    .with_system(walk::<Jacqueline, JacquelineConfig>)
                    .with_system(look::<Jacqueline>.run_if(cursor_showed))
                    .with_system(camera_shake::<Jacqueline, JacquelineConfig>)
                    .with_system(camera_roll::<Jacqueline, JacquelineConfig>)
                    .with_system(is_grounded::<Jacqueline, JacquelineConfig>)
                    .with_system(jump::<Jacqueline, JacquelineConfig>)
                    .with_system(pointing_on::<Jacqueline, JacquelineConfig>)

                    .with_system(attack)
                    // .with_system(fly::<Soul>)

                    .into(),
            );
    }
}

pub fn insert_other(mut commands: Commands, query: Query<Entity, Added<Soul>>) {
    for entity in query.iter() {
        commands
            .entity(entity);
    }
}
