use super::shared::{data_structs::go_history::History, resources::*, systems::*};
use crate::client::resources::*;
use crate::{client::plugin::ClientPipeline, server::plugin::ServerPipeline};

use bevy::{prelude::*, reflect::TypeRegistry};

use bevy_renet::{
     RenetClientPlugin, RenetServerPlugin,
};

use go_snap::*;

use std::{marker::PhantomData};
use go_snap::plugin::SnapType;

#[derive(Default, Debug)]
struct NetworkSnapshot;
// impl SnapType for NetworkSnapshot {
//     fn add_types(registry: &mut TypeRegistry) {
//         registry.write().register::<Transform>();
//         registry.write().register::<Id>();
//         registry.write().register::<Selected>();
//     }
// }


#[derive(Default)]
pub struct Reactive;

impl Plugin for Reactive {
    fn build(&self, mut app: &mut App) {
        //app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false));
        app.insert_resource(TickCount(0))
            //.insert_resource(Lobby::default())
            .insert_resource(SnapServer{ types: SyncedTypes {}})
            
            //.add_plugin(GoSnapPlugin::<SnapShot>::default())
           
            //.add_system(snap.with_run_criteria(bevy::core::FixedTimestep::step(1.)))
            // .insert_resource(RollbackType{
            //     type_registry: Vec::new()
            // })
            // .add_startup_system(add_id_provider)
            .add_system(setup_characters)
            //.add_system(create_room)
            //.add_plugin(GoSnap::<RollbackType>::default())
            // .add_system(snap)
            // .add_plugin(SnapPlugin::<SnapShot>::default())
            ;

        match is_server() {
            true => {
                println!("Game is running in server mode");
                app.insert_resource(new_renet_server())
                    .add_startup_system(setup_characters)
                    .add_plugin(RenetServerPlugin)
                    .add_plugin(ServerPipeline)
                    // .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
                    //     0.05,
                    // )))
                    ;
            }
            false => {
                println!("Game is running in client mode");
                app.insert_resource(new_renet_client())
                    .add_plugin(RenetClientPlugin)
                    .insert_resource(TickRate(Timer::from_seconds(2.0, false)))
                    .insert_resource(InternalShots(History::new(BUFFER_CAPACITY)))
                    .insert_resource(ServerShots(History::new(BUFFER_CAPACITY)))
                    .add_plugin(ClientPipeline)
                    .run();
            }
        }
    }
}
use super::rooms::*;

fn create_room(keys: Res<Input<KeyCode>>){
    if keys.just_pressed(KeyCode::R){
        init();
    }
}

