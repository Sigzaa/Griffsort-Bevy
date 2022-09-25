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
            .add_system_set(
                ConditionSet::new()
                    .with_system(insert_other)
                    .with_system(walk::<Soul, SoulConfig>)
                    .with_system(look::<Soul>.run_if(cursor_showed))
                    .with_system(camera_shake::<Soul, SoulConfig>)
                    .with_system(crosshair)
                    .with_system(camera_roll::<Soul, SoulConfig>)
                    .with_system(is_grounded::<Soul, SoulConfig>)
                    .with_system(jump::<Soul, SoulConfig>)
                    .with_system(shield_toggler)
                    .with_system(place_n_get_shield)
                    .with_system(pointing_on_shape::<Soul, SoulConfig>)
                    .with_system(crosshair)
                    .with_system(attack)
                    // .with_system(noclip_handler)
                    // .with_system(fly::<Soul>)
                    .with_system(sprint)
                    .into(),
            );
    }
}

pub fn insert_other(mut commands: Commands, query: Query<Entity, Added<Soul>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(ShieldCD(CDProps::default()))
            .insert(EscCD(CDProps::new(3)))
            .insert(ShieldUp(false))
            .insert(ShieldPos(None));
    }
}
