use bevy_simple_networking::{NetworkEvent, Transport};
use super::components::*;
use crate::game::components::{filters::*, player_data::*};
use bevy::{math::*, prelude::*};
use crate::game::networking::shared::additional::*;
    
pub fn handler(
    mut events: EventReader<NetworkEvent>,
    mut transport: ResMut<Transport>,
    mut con: ResMut<ConnectedList>,
    mut q_core: Query<(Entity, &mut Control, &Id, &mut Transform, &mut HeadRotation), With<Core>>,
    mut buf: ResMut<Buffer>,
    s_tick: Res<TickCounter>
) {
    
    for event in events.iter() {
        match event {
            NetworkEvent::Connected(handle) => {
                println!("client has been connected");
                match con.0.insert(*handle) {
                    // Sending id to the connected client.
                    Some(id) => transport.send("id", *handle, &id.to_string()),
                    None => {
                        error!("Server is full, no more free space")
                    } // move to client side // or send an Error to the client
                }
                con.0.print();
            }
            NetworkEvent::Disconnected(handle) => {
                con.0.remove(*handle);
                info!("{}: disconnected!", handle);
                con.0.print();
            }
            NetworkEvent::CliEvent(_handle, msg) => {
                // Listening for msg from clients to push it in the buffer.
                let msg_pack = msg_to_MsgPack(msg);
                // pushing input pack to the buffer.
                //println!("server tick: {:?}, client tick: {}", s_tick.0 , msg_pack.tick);
                buf.0.push(msg_pack.clone(), - msg_pack.tick); /* Magic trick to opposite priority. Maybe make it better for optimisation */
                
            }
            NetworkEvent::SendError(err, msg) => {
                error!(
                    "NetworkEvent::SendError (payload [{:?}]): {:?}",
                    msg.payload, err
                );
            }
            NetworkEvent::RecvError(err) => {
                error!("NetworkEvent::RecvError: {:?}", err);
            }
            _ => {}
        }
    }
}

