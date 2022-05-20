mod components;
mod connection_handler;
mod systems;
mod tick;

use crate::game::player_logic::client_controls::*;
use bevy::prelude::*;
use bevy_simple_networking::ClientPlugin;
use components::*;
use std::{
    env,
    net::{SocketAddr, UdpSocket},
};

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

        app.add_plugin(ClientPlugin)
            .insert_resource(remote_addr)
            .insert_resource(InputHistory(Vec::new()))
            .insert_resource(iter_count(0))
            .insert_resource(IsRollback(false))
            .insert_resource(InpBuf {
                pos: Vec3::ZERO,
                tick: -1,
            })
            .insert_resource(Tick(0))
            .insert_resource(ServerAddr(Vec::new()))
            .insert_resource(MyId(-5)) // That means that client has no binded id. TODO: Change to Enum
            .insert_resource(socket)
            .insert_resource(IsStarted(false))
            .insert_resource(TPS(Timer::from_seconds(2.0, false)))
            .add_system(connection_handler::handler)
            .add_system(smooth_camera)
            .add_plugin(tick::Tick)
            .run();
    }
}
