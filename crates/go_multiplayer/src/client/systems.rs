use bevy_simple_networking::NetworkEvent;
<<<<<<< HEAD:crates/go_multiplayer/src/client/systems.rs
use super::resources::*;
=======
use super::components::*;
use crate::game::components::{filters::*, player_data::*, *};
use crate::game::networking::additional::*;
>>>>>>> main:src/game/networking/client/connection_handler.rs
use bevy::{math::*, prelude::*};
use std::str;
use go_core::Character::*;

pub fn connection_handler(
    mut events: EventReader<NetworkEvent>,
    mut serv_addr: ResMut<ServerAddr>,
    mut selected_id: ResMut<SelectedId>,
    mut my_id: ResMut<MyId>,
) {
    for event in events.iter() {
        match event {
            NetworkEvent::SendError(err, msg) => {
                error!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            }
            NetworkEvent::RecvError(err) => {
                error!("NetworkEvent::RecvError: {:?}", err);
            }
            // discard irrelevant events
            NetworkEvent::CliEvent(handle, msg) => {
                if serv_addr.0.is_none() {
                    serv_addr.0  = Some(*handle);
                }
                
                todo!();
                
            }
            NetworkEvent::GetId(handle, msg) => {
                info!("I got my new id: {:?}, server ip: {}", msg, handle);
                let s = match str::from_utf8(msg) {
                    Ok(v) => v.parse().unwrap(),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                if serv_addr.0.is_none() {
                    serv_addr.0  = Some(*handle);
                }
                println!("msg: {:?}, s: {}", msg, s);
                my_id.0 = s;
                selected_id.0 = Some(s);
            }
            _ => {}
        }
    }
}
