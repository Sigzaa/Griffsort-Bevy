use bevy_simple_networking::{NetworkEvent, Transport};
use super::resources::*;
use bevy::{ prelude::*};
use crate::shared::resources::*;
use std::str;
use go_core::Character::*;
use crate::prelude::GoHistory;

pub(crate) fn connection_handler(
    mut q_core: Query<(&mut InputsBuffer, &Id), With<ChCore>>,
    mut events: EventReader<NetworkEvent>,
    mut transport: ResMut<Transport>,
    mut con: ResMut<ConnectedList>,
    s_tick: Res<TickCount>
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
                let msg_ser = str::from_utf8(&msg).unwrap();
                let pack: FromClient = serde_json::from_str(&msg_ser).unwrap();

                for (mut buffer, id) in q_core.iter_mut(){
                    if pack.id == id.0{
                        // pushing input pack to the buffer.
                        let input = pack.inputs.clone();
                        buffer.0.push(input, pack.tick);
                    }
                }
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
fn setup_players(
    mut commands: &mut Commands,
    mut q: Query<Entity, With<ChCore>>,
){
    for ent in q.iter_mut(){
        commands
        .entity(ent)
        .insert(InputsBuffer(GoHistory::<Inputs>::new(BUFFER_CAPACITY)));
    }
}

