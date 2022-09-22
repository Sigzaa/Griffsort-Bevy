pub mod resources;
mod abilities;

use actions::Actions;
use bevy_inspector_egui::InspectorPlugin;
use gs_states::{ConditionSet, IntoConditionalSystem, cursor_showed};
use synx::Synx;

use crate::Action;

use self::resources::JacquelineConfig;

use super::{*, default::*};

impl Plugin for Jacqueline{
    fn build(&self, app: &mut App) {
        app
        .add_plugin(InspectorPlugin::<JacquelineConfig>::new())
        //.add_plugin(Synx::<JacquelineConfig>::new("./config/soul.ron"))
            .add_system_set(
                ConditionSet::new()
                    // .with_system(walk::<Jacqueline, JacquelineConfig>)
                    // .with_system(look::<Jacqueline>.run_if(cursor_showed))
                    // .with_system(camera_shake::<Jacqueline, JacquelineConfig>)
                    // .with_system(camera_roll::<Jacqueline, JacquelineConfig>)
                    // .with_system(is_grounded::<Jacqueline, JacquelineConfig>)
                    // .with_system(jump::<Jacqueline, JacquelineConfig>)
                    // .with_system(pointing_on::<Jacqueline, JacquelineConfig>)

                    // // .with_system(noclip_handler)
                    // // .with_system(fly::<Soul>)

                    .into(),
            );
    }
}



impl Character<Jacqueline> for Jacqueline {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter()
        {
            if spawn_request.0 == "Jacqueline"
            {
                commands
                    .spawn()
                    .insert(Jacqueline)
                    .insert(CollisionGroups::new(0b01, 0b110))
                    .insert(Actions::<Action>::new())
                    .insert(PointingOn(Vec::new()))
                    .insert_bundle(States {
                        id: Id(spawn_request.2),
                        team: spawn_request.1.clone(),
                        ..Default::default()
                    });
            }
        }
    }
}