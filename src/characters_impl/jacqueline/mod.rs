mod abilities;
mod chase;
mod bomb;
mod marks;
pub mod resources;
mod shield;
use abilities::*;
use actions::Actions;

use bevy_inspector_egui::{widgets::{InspectorQuerySingle, InspectorQuery}, Inspectable, InspectorPlugin, egui};
use gs_states::{cursor_showed, ConditionSet, IntoConditionalSystem};

use crate::Action;

use self::{chase::ChaseAbil, marks::*, resources::*, shield::*, bomb::*};

use super::{default::*, *};

#[derive(Inspectable, Default)]
struct Config<Q: Inspectable + Default>{
    tranform: InspectorQuery<&'static mut Transform, With<Selected>>,
    _h: Q,
    //dynamic:

    //static:
}

#[derive(Inspectable, Default)]
struct Jacqueline_{
    
    max_hp: InspectorQuery<&'static mut MaxHp, With<Hero>>,
}


impl Plugin for Jacqueline {
    fn build(&self, app: &mut App) {
        app.add_event::<RecalkAnglesEv>()
            .add_event::<SpawnMarkEv>()
            .add_plugin(ChaseAbil)
            .add_plugin(InspectorPlugin::<Config::<Jacqueline_>>::new())
            .add_system(insert_marks)
            .add_system(respawn_marks.after(insert_marks))
            .add_system(spawn_mark.after(respawn_marks))
            .add_system(rearrange_angles.before(spawn_mark).label("last"))
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
                    .with_system(follow_hero)
                    .with_system(attack)
                    .with_system(idle_to_shield)
                    .with_system(mark_to_shield)
                    .with_system(shield_handler)
                    // .with_system(fly::<Soul>)
                    .into(),
            )
            .add_system_set(ConditionSet::new().after("main").into());
    }
}

pub fn insert_other(mut commands: Commands, query: Query<Entity, Added<Jacqueline>>) {
    for entity in query.iter()
    {
        commands
            .entity(entity)
            .insert(ShieldState {
                link: None,
                duration: 0.,
            })
            .insert(MarksCD(CDProps::new(0)));
    }
}
