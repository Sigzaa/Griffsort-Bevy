use super::shared::{data_structs::go_history::History, resources::*, systems::*};
use crate::client::resources::*;
use crate::{client::plugin::ClientPipeline, server::plugin::ServerPipeline};
use bevy::prelude::*;
use bevy_renet::{
    renet::{
        ConnectToken, RenetClient, RenetConnectionConfig, RenetServer, ServerConfig, ServerEvent,
        NETCODE_KEY_BYTES,
    },
    run_if_client_conected, RenetClientPlugin, RenetServerPlugin,
};
use corgee::*;
use bevy_snap::*;
use renet::RenetError;
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
const PROTOCOL_ID: u64 = 7;

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, mut app: &mut App) {
        //app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false));
        app.insert_resource(TickCount(0))
            .insert_resource(Lobby::default())
            .add_system(setup_characters);

        match is_server() {
            true => {
                println!("Game is running in server mode");
                app.insert_resource(new_renet_server())
                    .insert_resource(IsStarted(false))
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
                    .insert_resource(IsStarted(false))
                    .insert_resource(TickRate(Timer::from_seconds(2.0, false)))
                    .insert_resource(InternalShots(History::new(BUFFER_CAPACITY)))
                    .insert_resource(ServerShots(History::new(BUFFER_CAPACITY)))
                    .add_plugin(ClientPipeline)
                    .run();
            }
        }
    }
}

fn new_renet_client() -> RenetClient {
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

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(20, PROTOCOL_ID, server_addr, *PRIVATE_KEY);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}
fn setup_characters(query: Query<Entity, Added<NetSync>>, mut commands: Commands) {
    for ent in query.iter() {
        let mut ent_com = commands.entity(ent);
        ent_com
        .insert(InputsBuffer(History::<Inputs>::new(BUFFER_CAPACITY)));

        if is_server() {

        } else {

        }
    }
}
