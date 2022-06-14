use bevy_renet::{
    renet::{ConnectToken, RenetClient, RenetConnectionConfig, RenetServer, ServerConfig, ServerEvent, NETCODE_KEY_BYTES},
    run_if_client_conected, RenetClientPlugin, RenetServerPlugin,
};
use renet::RenetError;
use super::resources::*;
use bevy::{ prelude::*};
use crate::shared::resources::*;
use std::str;
use corgee::*;
use crate::prelude::History;

pub(crate) fn connection_handler(
    mut server_events: EventReader<ServerEvent>,
    mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Client {id} has been connected");
                for &player_id in lobby.players.keys() {
                    let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
                    server.send_message(*id, 0, message);
                }

                lobby.players.insert(*id, None);

                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
                server.broadcast_message(0, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Client {id} has been disconnected");
            }
        }
    }
}


pub(crate) fn receive_handler(
    mut server: ResMut<RenetServer>,
    mut q_char: Query<(&mut InputsBuffer, &Id), With<NetSync>>
){
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, 2) {
            match bincode::deserialize(&message).unwrap() {

                GenericMessages::ClientInputs { id, tick, inputs } => {
                    let arr_len = inputs.len();
                    let u_tick = tick as usize;

                    for (mut inp_buf, id) in q_char.iter_mut(){
                        for i in u_tick..(u_tick - arr_len){
                            inp_buf.0.insert(i as i32, inputs[i])
                        }
                    }
                }
                GenericMessages::Chat { id, tick } => {
                    todo!();
                }
                _ => info!("Received message type is undefined or invalid"),
            }
        }
    }
}



// pub(crate) fn connection_handler(
//     mut q_core: Query<(&mut InputsBuffer, &Id), With<ChCore>>,
//     mut events: EventReader<NetworkEvent>,
//     mut transport: ResMut<Transport>,
//     mut con: ResMut<ConnectedList>,
//     s_tick: Res<TickCount>
// ) {
//     for event in events.iter() {
//         match event {
//             NetworkEvent::Connected(handle) => {
//                 println!("client has been connected");
//                 match con.0.insert(*handle) {
//                     // Sending id to the connected client.
//                     Some(id) => transport.send("id", *handle, &id.to_string()),
//                     None => {
//                         error!("Server is full, no more free space")
//                     } // move to client side // or send an Error to the client
//                 }
//                 con.0.print();
//             }
//             NetworkEvent::Disconnected(handle) => {
//                 con.0.remove(*handle);
//                 info!("{}: disconnected!", handle);
//                 con.0.print();
//             }
//             NetworkEvent::CliEvent(_handle, msg) => {
//                 // Listening for msg from clients to push it in the buffer.
//                 let msg_ser = str::from_utf8(&msg).unwrap();
//                 let pack: FromClient = serde_json::from_str(&msg_ser).unwrap();

//                 for (mut buffer, id) in q_core.iter_mut(){
//                     if pack.id == id.0{
//                         // pushing input pack to the buffer.
//                         let input = pack.inputs.clone();
//                         buffer.0.push(input, pack.tick);
//                     }
//                 }
//             }
//             NetworkEvent::SendError(err, msg) => {
//                 error!(
//                     "NetworkEvent::SendError (payload [{:?}]): {:?}",
//                     msg.payload, err
//                 );
//             }
//             NetworkEvent::RecvError(err) => {
//                 error!("NetworkEvent::RecvError: {:?}", err);
//             }
//             _ => {}
//         }
//     }
// }
fn setup_players(
    mut commands: &mut Commands,
    mut q: Query<Entity, With<ChCore>>,
){
    for ent in q.iter_mut(){
        commands
        .entity(ent)
        .insert(InputsBuffer(History::<Inputs>::new(BUFFER_CAPACITY)));
    }
}

pub(crate) fn pop_buffer( // User should implement this (May be)
    mut q_core: Query<(&mut InputsBuffer, &mut GoInputs, &mut GoRot), With<ChCore>>,
    tick: ResMut<TickCount>,
) {
    for (mut inp_buf, mut ginp, mut grot) in q_core.iter_mut(){
        match inp_buf.0.get(tick.0) {
            Ok(input) => {
                *ginp = input.ginp;
                *grot = input.gorot;
            },
            Err(err) => {
                warn!("missed package at {}", tick.0);
            }
        }
    }
}

pub fn send_world(
    snap_shot: Res<SnapShot>,
    s_tick: ResMut<TickCount>,
    mut server: ResMut<RenetServer>,
) {
    
}

