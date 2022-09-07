mod abilities;
mod animation;
mod crosshair;
mod general;
mod resources;
mod sfx;
mod vfx;

use super::super::Action;
use iyes_loopless::prelude::*;
use bevy::{prelude::*, reflect::TypeUuid};
use super::*;
use corgee::*;
use abilities::*;
use crosshair::*;
use resources::*;
use general::*;
use sfx::*;
use vfx::*;
use actions::Actions;




impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app
        
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(crosshair_setup))
            .add_event::<ShieldEvent>()
            //.add_plugin(HotResource::<SoulConfig>::new("hee"))
            // .insert_resource(SoulConfig{
            //     should_render: true,
            //     ..Default::default()
            // })
            //.add_plugin(InspectorPlugin::<SoulConfig>::new())
            
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
                    .with_system(noclip_handler)
                    .with_system(fly::<Soul>)
                    .with_system(sprint)

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
                    .insert(ShieldCD(CDProps::default()))
                    .insert(EscCD(CDProps::new(3, 1.)))
                    .insert(Soul)
                    .insert(ShieldUp(false))
                    .insert(ShieldPos(None))
                    .insert(PreQTimer(PLACE_SHIELD))
                    .insert(QLimiter(true))
                    .insert(CollisionGroups::new(0b01, 0b110))
                    .insert(Actions::<Action>::new())
                    .insert_bundle(Config {
                        max_velocity: MaxSpeed(0.1),
                        max_jump_height: MaxJump(0.01),
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


    // fn load_config(mut commands: Commands, asset_server: Res<AssetServer>){
    //     asset_server.watch_for_changes().unwrap();
    //     let handle: Handle<SoulConfig> = asset_server.load("../config/config.ron");
    //     commands.insert_resource(handle);
    // }
    // fn apply_config<C: Bundle + TypeUuid + Clone>(    
    //     handle: Res<Handle<C>>,
    //     mut asset_config: ResMut<Assets<C>>,
    //     mut current_config: ResMut<C>,
    // ){
    //     if let Some(new_config) = asset_config.get_mut(&handle){
    //         *current_config = new_config.clone();
    //     } 
    // }
    
}

