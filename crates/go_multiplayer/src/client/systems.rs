use super::resources::*;
use crate::shared::resources::{GenericMessages, *};
use bevy::{math::*, prelude::*, core_pipeline::draw_2d_graph::input};
use bevy_renet::{
    renet::{
        ConnectToken, RenetClient, RenetConnectionConfig, RenetServer, ServerConfig, ServerEvent,
        NETCODE_KEY_BYTES,
    },
    run_if_client_conected, RenetClientPlugin, RenetServerPlugin,
};
use go_core::*;
use renet::RenetError;

/*

    Connection h.: channel 0,
    Error h.: channel 1,
    Receive h.: channel 2,

*/

pub(crate) fn receive_handler(mut commands: Commands, mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(2) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            GenericMessages::World { tick, snap } => {
                todo!();
            }
            GenericMessages::Chat { id, tick } => {
                todo!();
            }
            _ => info!("Received message type is undefined or invalid. Make sure you are using correct channel and enum"),
        }
    }
}

pub(crate) fn connection_handler(mut commands: Commands, mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(0) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            GenericMessages::PlayerConnected { id } => {
                todo!();
            }
            GenericMessages::PlayerDisconnected { id } => {
                todo!();
            }
            _ => info!("Received message type is undefined or invalid. Make sure you are using correct channel and enum"),
        }
    }
}

pub(crate) fn error_handler(){
    todo!();
}

pub(crate) fn send_input_history(
    mut q_selected: Query<(&Id, &mut GoInputs, &mut GoRot), With<Selected>>,
    mut client: ResMut<RenetClient>,
    tick: ResMut<TickCount>,
) {
    for (id, ginp, gorot) in q_selected.iter_mut() {

        let client_inputs = GenericMessages::ClientInputs {
            id: id.0,
            tick: tick.0,
            inputs: [Inputs {
                ginp: *ginp,
                gorot: *gorot,
            }; 4],
        };

        let input_message = bincode::serialize(&client_inputs).unwrap();
        client.send_message(0, input_message);
    }
}
pub(crate) fn send_chat(){
    todo!();
}
pub(crate) fn is_desync(external_buf: ResMut<ServerShots>, internal_buf: ResMut<InternalShots>) {
    let tick = match external_buf.0.last_tick() {
        Some(tick) => tick,
        None => return,
    };

    match (&external_buf.0.get(tick), &internal_buf.0.get(tick)) {
        (Ok(ext_content), Ok(int_content)) => {
            if ext_content != int_content {
                warn!("Desync on {}", tick);
            }
        }
        _ => (),
    }
    todo!();
}

// pub fn connection_handl1er(
//     mut events: EventReader<NetworkEvent>,
//     mut serv_addr: ResMut<ServerAddr>,
//     mut selected_id: ResMut<SelectedId>,
//     mut my_id: ResMut<MyId>,
// ) {
//     for event in events.iter() {
//         match event {
//             NetworkEvent::SendError(err, msg) => {
//                 error!(
//                     "NetworkEvent::SendError (payload [{:?}]): {:?}",
//                     msg.payload, err
//                 );
//             }
//             NetworkEvent::RecvError(err) => {
//                 error!("NetworkEvent::RecvError: {:?}", err);
//             }
//             // discard irrelevant events
//             NetworkEvent::CliEvent(handle, msg) => {
//                 if serv_addr.0.is_none() {
//                     serv_addr.0 = Some(*handle);
//                 }

//                 todo!();
//             }
//             NetworkEvent::GetId(handle, msg) => {
//                 info!("I got my new id: {:?}, server ip: {}", msg, handle);
//                 let s = match str::from_utf8(msg) {
//                     Ok(v) => v.parse().unwrap(),
//                     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
//                 };
//                 if serv_addr.0.is_none() {
//                     serv_addr.0 = Some(*handle);
//                 }
//                 println!("msg: {:?}, s: {}", msg, s);
//                 my_id.0 = s;
//                 selected_id.0 = Some(s);
//             }
//             _ => {}
//         }
//     }
// }
