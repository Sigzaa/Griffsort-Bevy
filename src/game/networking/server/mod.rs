pub mod components;
mod connection_handler;
mod tick;
pub mod systems;

use crate::game::components::{ *};
use bevy::{core::FixedTimestep, prelude::*};
use std::{env, str, net::{ UdpSocket }};
use components::*;

use tick::*;
use super::shared::a_list::AList;
use bevy_simple_networking::{ ServerPlugin };
use crate::game::networking::shared::resources::*;

use bevy::{ app::ScheduleRunnerSettings };
use priority_queue::PriorityQueue;

use std::{ time::Duration};


#[derive(Default)]
pub struct IsStarted(pub bool);

pub struct Server;
impl Plugin for Server {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = env::args().collect();
        if &args[1] != "server" {
            return;
        }
        let listen_address: &str = &args[2].to_owned();
        let socket = UdpSocket::bind(listen_address).expect("could not bind socket");
        socket
            .set_nonblocking(true)
            .expect("could not set socket to be nonblocking");
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("could not set read timeout");

        info!("Server now listening on {}", listen_address);

        app 
            .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
                0.05,
            )))
            .insert_resource(socket)
            .insert_resource(Buffer(PriorityQueue::new()))
            .insert_resource(ConnectedList(AList::default()))
            .insert_resource(Timer1(0.))
            .insert_resource(components::IsStarted(false))
            .insert_resource(TickCounter(-50))
            //.add_plugins(MinimalPlugins)
            //.add_plugin(LogPlugin)
            
            
            .add_system(connection_handler::handler.label("msg_collect"))
           
            //.add_system(send_message)
            .add_plugin(ServerPlugin)
            .add_plugin(tick::Tick)
            .run();
    }
}
