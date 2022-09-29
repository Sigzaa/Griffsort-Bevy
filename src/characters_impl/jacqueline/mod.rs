mod abilities;
mod marks;
pub mod resources;
mod shield;

use std::{time::Duration, collections::HashSet};

use abilities::*;
use actions::Actions;
use bevy_inspector_egui::{widgets::InspectorQuerySingle, InspectorPlugin};
use gs_states::{cursor_showed, ConditionSet, IntoConditionalSystem};
use keyframe::num_traits::ToPrimitive;
use synx::Synx;

use crate::Action;

use self::{marks::*, resources::*, shield::*};

use super::{default::*, *};

impl Plugin for Jacqueline {
    fn build(&self, app: &mut App) {
        app.add_event::<RecalkAnglesEv>()
            .add_event::<SpawnMarkEv>()
            .add_system_set(
                ConditionSet::new()
                    .label("main")
                    .with_system(insert_other)
                    .with_system(insert_marks)
                    .with_system(spawn_mark)
                    .with_system(respawn_marks)
                    .with_system(setup_shield)
                    .with_system(walk::<Jacqueline, JacquelineConfig>)
                    .with_system(look::<Jacqueline>.run_if(cursor_showed))
                    .with_system(camera_shake::<Jacqueline, JacquelineConfig>)
                    .with_system(camera_roll::<Jacqueline, JacquelineConfig>)
                    .with_system(is_grounded::<Jacqueline, JacquelineConfig>)
                    .with_system(jump::<Jacqueline, JacquelineConfig>)
                    .with_system(pointing_on::<Jacqueline, JacquelineConfig>)
                    .with_system(follow_hero)
                    .with_system(attack)
                    .with_system(idle_to_shield)
                    .with_system(mark_to_shield)
                    .with_system(shield_handler)
                    .with_system(rearrange_angles)
                    // .with_system(fly::<Soul>)
                    .into(),
            )
            .add_system_set(
                ConditionSet::new()
                    .after("main")
                    
                    .into(),
            );
    }
}

pub fn insert_other(
    mut commands: Commands,
    query: Query<Entity, Added<Jacqueline>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    conf: Res<JacquelineConfig>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(ShieldState {
                link: None,
                duration: 0.,
            })
            .insert(MarksCD(CDProps::new(0)))
            ;
    }
}
