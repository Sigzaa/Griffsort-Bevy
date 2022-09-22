mod abilities;
mod animation;
mod crosshair;
mod general;
pub mod resources;
mod sfx;
mod vfx;

use super::*;
use super::{super::Action, default::*};
use abilities::*;
use actions::Actions;
use bevy::{prelude::*, reflect::TypeUuid};
use bevy_inspector_egui::InspectorPlugin;
use crosshair::*;
use general::*;
use gs_states::{cursor_showed, GameState};
use iyes_loopless::prelude::*;
pub use resources::*;
use sfx::*;
use synx::Synx;
use vfx::*;

impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::InGame, crosshair_setup)
            .add_event::<ShieldEvent>()
            .add_plugin(InspectorPlugin::<SoulConfig>::new())
            //.insert_resource(SoulConfig::default())
            .add_plugin(Synx::<SoulConfig>::new("./config/soul.ron"))
            .add_system_set(
                ConditionSet::new()
                    .with_system(walk::<Soul, SoulConfig>)
                    .with_system(look::<Soul>.run_if(cursor_showed))
                    .with_system(camera_shake::<Soul, SoulConfig>)
                    .with_system(crosshair)
                    .with_system(camera_roll::<Soul, SoulConfig>)
                    .with_system(is_grounded::<Soul, SoulConfig>)
                    .with_system(jump::<Soul, SoulConfig>)
                    .with_system(shield_toggler)
                    .with_system(place_n_get_shield)
                    .with_system(pointing_on::<Soul, SoulConfig>)
                    .with_system(crosshair)
                    .with_system(attack)
                    // .with_system(noclip_handler)
                    // .with_system(fly::<Soul>)
                    .with_system(sprint)
                    .into(),
            );
    }
}

impl Character<Soul> for Soul {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter()
        {
            if spawn_request.0 == "Soul"
            {
                commands
                    .spawn()
                    .insert(ShieldCD(CDProps::default()))
                    .insert(EscCD(CDProps::new(3)))
                    .insert(Soul)
                    .insert(ShieldUp(false))
                    .insert(ShieldPos(None))
                    .insert(CollisionGroups::new(0b01, 0b110))
                    .insert(Actions::<Action>::new())
                    .insert(PointingOn(Vec::new()))
                    .insert_bundle(States {
                        id: Id(spawn_request.2),
                        team: Team::Dark,
                        ..Default::default()
                    });
            }
        }
    }
}
