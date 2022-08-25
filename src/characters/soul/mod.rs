mod abilities;
mod anim;
mod crosshair;
mod general;
mod resources;
mod sfx;
mod vfx;

use iyes_loopless::prelude::*;
use bevy::prelude::*;
use super::*;
use corgee::*;
use abilities::*;
use crosshair::*;
use resources::*;
use general::*;
use sfx::*;
use vfx::*;

impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(crosshair_setup))
            .add_event::<ShieldEvent>()
            .add_system_set(
                ConditionSet::new()
                    .with_system(walk::<Soul>)
                    .with_system(look::<Soul>)
                    .with_system(camera_shake::<Soul>)
                    .with_system(camera_roll::<Soul>)
                    .with_system(is_grounded::<Soul>)
                    .with_system(jump::<Soul>)
                    .with_system(shield_toggler)
                    .with_system(place_n_get_shield)
                    .with_system(is_pointing)
                    .with_system(crosshair)
                    .with_system(attack)
                    .into(),
            );
    }
}


impl Character<Soul> for Soul {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter() {
            if spawn_request.0 == "Soul" {
                commands
                    .spawn()
                    .insert(Soul)
                    .insert(ShieldUp(false))
                    .insert(ShieldPos(None))
                    .insert(PreQTimer(PLACE_SHIELD))
                    .insert(QLimiter(true))
                    .insert(CollisionGroups::new(0b01, 0b110))
                    .insert_bundle(Config {
                        max_velocity: MaxSpeed(0.1),
                        ..Default::default()
                    })
                    .insert_bundle(States {
                        id: Id(spawn_request.2),
                        team: Team(spawn_request.1 as i16),
                        ..Default::default()
                    });
            }
        }
    }
}