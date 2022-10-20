use bevy_renet::{
    renet::{ RenetServer, ServerEvent},
};

use bevy::{ prelude::*};
use crate::shared::resources::*;
use crate::shared::data_structs::go_history::*;
use crate::prelude::History;

pub(crate) fn connection_handler(
    mut server_events: EventReader<ServerEvent>,
   // mut lobby: ResMut<Lobby>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _) => {
                println!("Player {id} connected.");
                for &player_id in lobby.players.keys() {
                    let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
                    server.send_message(*id, 0, message);
                }

                lobby.players.insert(*id, None);

                let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
                server.broadcast_message(0, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                println!("Player {id} disconnected.");
                lobby.players.remove(id);

                let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *id }).unwrap();
                server.broadcast_message(0, message);
            }
        }
    }
}


pub(crate) fn receive_handler(
    mut server: ResMut<RenetServer>,
    mut q_char: Query<(&mut InputsBuffer, &Id), With<NetSync>>
){
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, 0) {
            
            match bincode::deserialize(&message).unwrap() {              
                GenericMessages::ClientInputs { id, tick, inputs } => {
                    let arr_len = inputs.len();
                    let u_tick = tick as usize;
                    for (mut inp_buf, _id) in q_char.iter_mut(){
                        for i in 0..arr_len{
                            
                            inp_buf.0.insert( tick - (i as i32), inputs[i]);
                        }
                        println!("buffer: {}", inp_buf.0.map.len());
                    }   
                }
                GenericMessages::Chat { tick } => {

                }
                _ => warn!("Received message type is undefined or invalid"),
            }
        }
    }
}

pub(crate) fn pop_buffer( // User should implement this (May be)
    mut q_core: Query<(&mut InputsBuffer, &mut GoInputs, &mut GoRot), With<NetSync>>,
    tick: ResMut<TickCount>,
) {
    for (inp_buf, mut ginp, mut grot) in q_core.iter_mut(){
        match inp_buf.0[tick.0] {
            Bu::Gen(input) => {
                *ginp = input.ginp;
                *grot = input.gorot;
            },
            Bu::Empty => {
                warn!("Loss packet at {}", tick.0);
            }
        }
    }
}

// pub fn send_world(
//     snap_shot: Res<SnapShot>,
//     s_tick: ResMut<TickCount>,
//     mut server: ResMut<RenetServer>,
// ) {
    
// }



// pub(crate) fn connection_handler(
//     mut q_core: Query<(&mut InputsBuffer, &Id), With<Hero>>,
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

