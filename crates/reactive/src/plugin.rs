use super::shared::{data_structs::go_history::History, resources::*, systems::*};
use crate::client::resources::*;
use crate::{client::plugin::ClientPipeline, server::plugin::ServerPipeline};

use bevy::{prelude::*, reflect::TypeRegistry};

use bevy_renet::{
     RenetClientPlugin, RenetServerPlugin,
};

use corgee::*;
use go_snap::*;
use bevy_snap::*;

use std::{marker::PhantomData};
use go_snap::plugin::SnapType;


fn snap(mut commands: Commands, keys: Res<Input<KeyCode>>) {

        info!("Making snap");
        commands.save::<SnapShot>();
    
}

fn store_snapshot(
    mut save_events: EventReader<SaveEvent<SnapShot>>,
    mut entity: Query<Entity, With<Selected>>,
    mut commands: Commands

) {
    for save_event in save_events.iter() {
        info!("Writing snapshot to save slot resource");

        // for ent in entity.iter(){
        //     let com_ent = commands.entity(ent);
        //     //println!("com_ent: {:?}", com_ent);
        // }

        // Save the snapshot in a resource so we can restore it later
        // let snap_diff = snap1 - snap2;

        // snap.get_checksum(id);
        // let vel = snap.get::<Velocity>(id);
        //let encoded: Vec<u8> = bincode::serialize(&save_event.snapshot.clone()).unwrap();

        //let des = serde_json::to_string(&save_event.snapshot.clone()).unwrap();
        //println!("{:?}", &save_event.snapshot.clone());
    }
}


fn add_id_provider(
    q: Query<Entity, With<NetSync>>,
    mut snapshot_id_provider: ResMut<SnapshotIdProvider<SnapShot>>,
    mut commands: Commands,
){
    for ent in q.iter(){
        commands.entity(ent).insert(snapshot_id_provider.next());
    }
}

#[derive(Default, Debug)]
struct NetworkSnapshot;
impl SnapType for NetworkSnapshot {
    fn add_types(registry: &mut TypeRegistry) {
        registry.write().register::<Transform>();
        registry.write().register::<Id>();
        registry.write().register::<Selected>();
    }
}


#[derive(Default)]
pub struct Reactive;

impl Plugin for Reactive {
    fn build(&self, mut app: &mut App) {
        //app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false));
        app.insert_resource(TickCount(0))
            .insert_resource(Lobby::default())
            .insert_resource(SnapServer{ types: SyncedTypes {}})
            .add_startup_system(add_id_provider)
            .add_system(store_snapshot)
            //.add_plugin(GoSnapPlugin::<SnapShot>::default())
            .add_plugin(SnapPlugin::<SnapShot>::default())
            //.add_system(snap.with_run_criteria(bevy::core::FixedTimestep::step(1.)))
            // .insert_resource(RollbackType{
            //     type_registry: Vec::new()
            // })
            // .add_startup_system(add_id_provider)
            .add_system(setup_characters)
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

