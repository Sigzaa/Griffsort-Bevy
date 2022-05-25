use bevy_simple_networking::NetworkEvent;
use super::resources::*;
use bevy::{math::*, prelude::*};
use std::str;
use gocha::prelude::*;

pub fn connection_handler(
    mut events: EventReader<NetworkEvent>,
    mut serv_addr: ResMut<ServerAddr>,
    mut binded_id: ResMut<BindedId>,
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
                if serv_addr.0.len() <= 0 {
                    serv_addr.0.push(*handle);
                }
                
                todo!();
                
            }
            NetworkEvent::GetId(handle, msg) => {
                info!("I got my new id: {:?}, server ip: {}", msg, handle);
                let s = match str::from_utf8(msg) {
                    Ok(v) => v.parse().unwrap(),
                    Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                };
                serv_addr.0.push(*handle);
                println!("msg: {:?}, s: {}", msg, s);
                my_id.0 = s;
                binded_id.0 = s;
            }
            _ => {}
        }
    }
}
