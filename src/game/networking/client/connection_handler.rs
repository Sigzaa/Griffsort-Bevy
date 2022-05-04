use bevy_simple_networking::NetworkEvent;
use super::components::*;
use crate::game::components::{ *};
use crate::game::networking::shared::additional::*;
use bevy::{math::*, prelude::*};
use std::str;

pub fn handler(
    mut events: EventReader<NetworkEvent>,
    mut serv_addr: ResMut<ServerAddr>,
    mut binded_id: ResMut<BindedId>,
    mut my_id: ResMut<MyId>,
    mut inp_buf: ResMut<InpBuf>,
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
                //info!("server sent a message: {:?}", msg);
                let inp = split(msg);
                let id = inp[0];
                let tick = inp[1] as i32;
                let pos = Vec3::new(inp[2], inp[3], inp[4]);

                if id as i32 == my_id.0 && inp_buf.tick < tick {
                    inp_buf.pos = pos;
                    inp_buf.tick = tick;
                }
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
