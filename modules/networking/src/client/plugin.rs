use crate::prelude::go_history::*;
use crate::shared::resources::*;
use bevy::prelude::*;
use bevy_simple_networking::ClientPlugin;
use super::resources::*;
use super::systems::*;
use super::tick::plugin::Tick;
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
            .insert_resource(IterCount(0))
            .insert_resource(IsRollback(false))
            .insert_resource(TickCount(0))
            .insert_resource(ServerAddr(Vec::new()))
            .insert_resource(MyId(-5)) // That means that client has no binded id. TODO: Change to Enum
            .insert_resource(socket)
            .insert_resource(IsStarted(false))
            .insert_resource(TPS(Timer::from_seconds(2.0, false)))
            .insert_resource(InternalShots(GoHistory::new(200)))
            .insert_resource(ServerShots(GoHistory::new(BUFFER_CAPACITY)))
            .add_system(connection_handler)
            .add_plugin(Tick)
            .run();
    }
}