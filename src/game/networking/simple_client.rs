use std::net::{SocketAddr, UdpSocket};
use std::env;
use bevy::{log::LogPlugin, prelude::*};
use bevy_simple_networking::{ClientPlugin, NetworkEvent};
pub struct Client;
impl Plugin for Client {
    fn build(&self, app: &mut App) {
    //println!("args: {}, {}", &args[0], &args[1]);
    
        let args: Vec<String> = env::args().collect();
        if &args[1] != "client" { return; }
        
        let remote_addr: SocketAddr = "127.0.0.1:4567".parse().expect("could not parse addr");
        let socket = UdpSocket::bind("[::]:0").expect("could not bind socket");
        socket
            .connect(remote_addr)
            .expect("could not connect to server");
        socket
            .set_nonblocking(true)
            .expect("could not set socket to be nonblocking");
            
        app
            .insert_resource(remote_addr)
            .insert_resource(socket)
            //.add_plugins(MinimalPlugins)
            //.add_plugin(LogPlugin)
            .add_plugin(ClientPlugin)
            .add_system(connection_handler)
            .run();
            
    }
}
fn connection_handler(mut events: EventReader<NetworkEvent>) {
    for event in events.iter() {
        match event {
            NetworkEvent::Message(_, msg) => {
                info!("server sent a message: {:?}", msg);
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
            // discard irrelevant events
            _ => {}
        }
    }
}
