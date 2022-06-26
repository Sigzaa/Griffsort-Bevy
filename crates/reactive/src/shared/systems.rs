use bevy::{prelude::*};
use super::{resources::*, data_structs::go_history::History};
use bevy::ecs::schedule::ShouldRun;
use bevy_renet::{
    renet::{
        ConnectToken, RenetClient, RenetConnectionConfig, RenetServer, ServerConfig,
        NETCODE_KEY_BYTES,
    },
};

use std::{net::UdpSocket, time::SystemTime};

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
const PROTOCOL_ID: u64 = 7;

pub fn update_tick(mut tick: ResMut<TickCount>) {
    tick.0 += 1;
}

pub fn is_server() -> bool{
    let args: Vec<String> = std::env::args().collect();
    
    let exec_type = &args[1];
    return match exec_type.as_str() {
        "server" => true,
        _ => false,
    };
}
pub fn run_if_server() -> ShouldRun {
    let args: Vec<String> = std::env::args().collect();
    let exec_type = &args[1];
    return match exec_type.as_str() {
            "server" => ShouldRun::Yes,
            _ => ShouldRun::No,
    };
}
pub(crate) fn new_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = RenetConnectionConfig::default();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    // This connect token should come from another system, NOT generated from the client.
    // Usually from a matchmaking system
    // The client should not have access to the PRIVATE_KEY from the server.
    let token = ConnectToken::generate(
        current_time,
        PROTOCOL_ID,
        300,
        client_id,
        15,
        vec![server_addr],
        None,
        PRIVATE_KEY,
    )
    .unwrap();
    RenetClient::new(current_time, socket, client_id, token, connection_config).unwrap()
}

pub(crate) fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(20, PROTOCOL_ID, server_addr, *PRIVATE_KEY);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
pub(crate) fn setup_characters(query: Query<Entity, Added<NetSync>>, mut commands: Commands) {
    for ent in query.iter() {
        let mut ent_com = commands.entity(ent);
        ent_com
        .insert(InputsBuffer(History::<Inputs>::new(BUFFER_CAPACITY)));

        if is_server() {

        } else {

        }
    }
}


// pub fn save_snap(mut commands: Commands) {
//         // This triggers saving the world the next time commands are processed.
//         // The snapshot is then sent as an event so it can be picked up by other systems.
//         commands.save::<SnapShot>();

// }

// pub(crate) fn store_snapshot(
//     mut save_events: EventReader<SaveEvent<SnapShot>>,
//     mut save_slot: ResMut<SnapBuffer>,
//     tick: Res<TickCount>,
// ) {
//     for save_event in save_events.iter() {


//         // Save the snapshot in a resource so we can restore it later
//         //save_slot.0.push(save_event.snapshot.clone(), tick.0);

//     }
// }
// pub(crate) fn load_snap(mut commands: Commands, keys: Res<Input<KeyCode>>, save_slot: ResMut<SnapBuffer>){
//     if keys.just_pressed(KeyCode::E) {
        
//         //commands.load::<SnapShot>(save_slot.0[TICKRATE as usize].clone())
//     }
// }