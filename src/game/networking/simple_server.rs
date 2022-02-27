use super::bevy_simple_networking::{NetworkEvent, NetworkResource, ServerPlugin, Transport};
use crate::game::components::*;
use crate::game::components::{filters::*, player_data::*, *};
use bevy::core::FixedTimestep;
use bevy::{app::ScheduleRunnerSettings, log::LogPlugin, prelude::*};
use bytes::Bytes;
use std::env;
use std::{io, net::UdpSocket, time::Duration};

//const LISTEN_ADDRESS: &str = "127.0.0.1:4567";
const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;

pub struct Server;
impl Plugin for Server {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = env::args().collect();
        if &args[1] != "server" {
            return;
        }
        let LISTEN_ADDRESS: &str = &args[2].to_owned();
        let socket = UdpSocket::bind(LISTEN_ADDRESS).expect("could not bind socket");
        socket
            .set_nonblocking(true)
            .expect("could not set socket to be nonblocking");
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("could not set read timeout");

        info!("Server now listening on {}", LISTEN_ADDRESS);

        app
            // run the server at a reduced tick rate (100 ticks per minute)
            .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
                0.05,       
            )))
            .insert_resource(socket)
            .insert_resource(ConnectedList(Vec::new()))
            //.add_plugins(MinimalPlugins)
            //.add_plugin(LogPlugin)
            .add_plugin(ServerPlugin)
            .add_system(connection_handler)
            .add_system(send_message)
            .run();
    }
}

fn connection_handler(
    mut events: EventReader<NetworkEvent>,
    mut transport: ResMut<Transport>,
    mut connected_list: ResMut<ConnectedList>,
    mut q_cores: Query<(&mut Control, &Id, &mut Transform, &mut HeadRotation), With<Core>>,
) {
    if connected_list.0.len() >= 1 {
        //transport.send(connected_list.0[0], b"hi");
    }
    //
    for event in events.iter() {
        //println!("hello");
        match event {
            NetworkEvent::Connected(handle) => {
                connected_list.0.push(*handle);
                info!("{}: connected!", handle);
                transport.send(*handle, b"1");
            }
            NetworkEvent::Disconnected(handle) => {
                connected_list.0.retain(|&x| x != *handle);
                info!("{}: disconnected!", handle);
            }
            NetworkEvent::Message(handle, msg) => {
                //info!("{} sent a message: {:?}", handle, msg);

                let input: Vec<f32> = String::from_utf8_lossy(msg)
                    .split_whitespace()
                    .map(|s| s.parse().expect("parse error"))
                    .collect();
                //println!("received");
                //println!("input: {:?}", input);

                for (mut ctrl, id, mut transform, mut head_rotation) in q_cores.iter_mut() {
                    if id.0 == input[0] as i32 {
                        //ctrl.delta_x = input[1];
                        //ctrl.delta_y = input[2];
                        ctrl.forward = to_bool(input[3]);
                        ctrl.left = to_bool(input[4]);
                        ctrl.right = to_bool(input[5]);
                        ctrl.back = to_bool(input[6]);
                        ctrl.q = to_bool(input[7]);
                        ctrl.lmb = to_bool(input[8]);
                        ctrl.rmb = to_bool(input[9]);
                        ctrl.jump = to_bool(input[10]);
                        ctrl.shift = to_bool(input[11]);
                        ctrl.e = to_bool(input[12]);
                        let vec = Vec3::new(input[13], input[14], input[15]);
                        let head_vec = Vec3::new(input[16], input[17], input[18]);

                        transform.rotation = Quat::from_scaled_axis(vec);
                        head_rotation.0 = Quat::from_scaled_axis(head_vec);
                    }


                    //ctrl.forward = message.starts_with("t");
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
#[derive(Default)]
pub struct ConnectedList(pub Vec<std::net::SocketAddr>);


fn send_message(
    net: Res<NetworkResource>,
    mut transport: ResMut<Transport>,
    q_core: Query<(&Id, &Transform, &HeadRotation, &Control), With<Core>>,
    mut connected_list: ResMut<ConnectedList>,
) {
    
        for (id, transform, head_rotation, ctrl) in q_core.iter() {
            let axis = transform.rotation.to_scaled_axis();
            let head_axis = head_rotation.0.to_scaled_axis();
            let message = format!(
                "{} {} {} {} {} {} {} {} {} {} {} {}",
                id.0,
                transform.translation[0],
                transform.translation[1],
                transform.translation[2],
                axis[0],
                axis[1],
                axis[2],
                head_axis[0],
                head_axis[1],
                head_axis[2],
                i8::from(ctrl.q),
                i8::from(ctrl.lmb),

            );
            //println!("here we are");
            //println!("connected: {:?}", connected_list.0);
            for addr in &connected_list.0{

                transport.send(*addr, message.as_bytes());
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
