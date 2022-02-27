use super::bevy_simple_networking::{ClientPlugin, NetworkEvent, Transport};
use crate::game::components::{filters::*, player_data::*, *};
use bevy::{log::LogPlugin, prelude::*};
use std::env;
use toml::{Value, de::Error};
use std::fs;
use std::net::{SocketAddr, UdpSocket};
pub struct Client;
impl Plugin for Client {
    fn build(&self, app: &mut App) {


        let args: Vec<String> = env::args().collect();
        if &args[1] != "client" {
            return;
        }
        let remote_addr: SocketAddr = args[2].parse().expect("could not parse addr");
        let socket = UdpSocket::bind("0.0.0.0:0").expect("could not bind socket");
        socket
            .connect(remote_addr)
            .expect("could not connect to server");
        socket
            .set_nonblocking(true)
            .expect("could not set socket to be nonblocking");

        app.insert_resource(remote_addr)
            .insert_resource(ServerAddr(Vec::new()))
            .insert_resource(socket)
            //.add_plugins(MinimalPlugins)
            //.add_plugin(LogPlugin)
            .add_plugin(ClientPlugin)
            .add_system(connection_handler)
            .add_system(send_message)
            .add_system(receive_message)
            .run();
    }
}

#[derive(Default)]
struct ServerAddr(Vec<std::net::SocketAddr>); // Replace Vec by something better.

fn connection_handler(
    mut events: EventReader<NetworkEvent>,
    mut transport: ResMut<Transport>,
    q_selected: Query<(&Id, &Control), With<Selected>>,
    mut serv_addr: ResMut<ServerAddr>,
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
            _ => {}
        }
    }
}

fn send_message(
    mut transport: ResMut<Transport>,
    q_selected: Query<(&Id, &Control, &Transform, &HeadRotation), With<Selected>>,
    mut serv_addr: ResMut<ServerAddr>,
) {
    for (id, ctrl, transform, head_rotation) in q_selected.iter() {
        let axis = transform.rotation.to_scaled_axis();
        let head_axis = head_rotation.0.to_scaled_axis();
        if serv_addr.0.len() >= 1 {
            let message = format!(
                "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
                //1, // 1 - Events, 2 - Chat,
                id.0,
                ctrl.delta_x,
                ctrl.delta_y,
                i8::from(ctrl.forward),
                i8::from(ctrl.left),
                i8::from(ctrl.right),
                i8::from(ctrl.back),
                i8::from(ctrl.q),
                i8::from(ctrl.lmb),
                i8::from(ctrl.rmb),
                i8::from(ctrl.jump),
                i8::from(ctrl.shift),
                i8::from(ctrl.e),
                axis[0],
                axis[1],
                axis[2],
                head_axis[0],
                head_axis[1],
                head_axis[2],
            );
            //println!("id: {}", id.0);
            transport.send(serv_addr.0[0], message.as_bytes());
        }
    }
}

fn receive_message(
    mut events: EventReader<NetworkEvent>,
    mut serv_addr: ResMut<ServerAddr>,
    mut q_core: Query<(&mut Control, &Id, &mut Transform, Entity, &mut HeadRotation), With<Core>>,
    sel_id: Res<BindedId>,
) {
    for event in events.iter() {
        match event {
            NetworkEvent::Message(handle, msg) => {
                if serv_addr.0.len() <= 0 {
                    serv_addr.0.push(*handle);
                }
                //info!("server sent a message: {:?}", msg);
                let input: Vec<f32> = String::from_utf8_lossy(msg)
                    .split_whitespace()
                    .map(|s| s.parse().expect("parse error"))
                    .collect();
                //println!("input: {:?}", input);
                if input.len() <= 3{ return; }
                for (mut ctrl, id, mut transform, ent, mut head_rotation) in q_core.iter_mut() {
                    if input[0] == id.0 as f32 {
                        transform.translation[0] = input[1];
                        transform.translation[1] = input[2];
                        transform.translation[2] = input[3];
                        if sel_id.0 as f32 != input[0]{
                            let head_vec = Vec3::new(input[7], input[8], input[9]);
                            head_rotation.0 = Quat::from_scaled_axis(head_vec);
                            ctrl.q = to_bool(input[10]);
                            ctrl.lmb = to_bool(input[11]);
                            let vec = Vec3::new(input[4], input[5], input[6]);
                            transform.rotation = Quat::from_scaled_axis(vec);
                        }
                        
                    }
                }
            }
            _ => {}
        }
    }
}
fn to_bool(num: f32) -> bool {
    if num == 0. {
        false
    } else {
        true
    }
}
