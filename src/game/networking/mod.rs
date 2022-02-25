use crate::game::components::{filters::*, player_states::*, *};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bootleg_networking::*;
use std::env;
use std::sync::Arc;
use std::io::{self, BufRead};   

const MESSAGE_CHANNEL_ID: MessageChannelID = MessageChannelID::new(0);
const MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: MESSAGE_CHANNEL_ID.id,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 256,
    packet_buffer_size: 256,
};

mod client;
mod server;

use bevy::core::FixedTimestep;

// The timestep says how many times to run the SystemSet every second
// For TIMESTEP_1, it's once every second
// For TIMESTEP_2, it's twice every second

const TIMESTEP_1_PER_SECOND: f64 = 1.;
const TIMESTEP_2_PER_SECOND: f64 = 30.0 / 60.0;

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, app: &mut App) {
        app.add_plugin(NetworkingPlugin)
            .add_startup_system(setup)
            .add_system(client::send)
            .add_system(client::receive)
            .add_system(server::receive)
            .add_system_set(
                SystemSet::new()
                    // This prints out "hello world" once every second
                    .with_run_criteria(FixedTimestep::step(TIMESTEP_1_PER_SECOND))
                    .with_system(server::send)
            )
            
            .run();
    }
}

fn setup(mut commands: Commands, tokio_rt: Res<Runtime>, task_pool: Res<IoTaskPool>) {
    let args: Vec<String> = env::args().collect();
    //println!("args: {}, {}", &args[0], &args[1]);
    if &args[1] == "server" {
        println!("setuping server");
        let mut net = NetworkResource::new_server(tokio_rt.clone(), task_pool.0.clone());
        let listen_config = ListenConfig {
            tcp_addr: "127.0.0.1:9000",
            udp_addr: "127.0.0.1:9001",
            naia_addr: "127.0.0.1:9003",
            webrtc_listen_addr: "127.0.0.1:9004",
            public_webrtc_listen_addr: "127.0.0.1:9004",
        };

        net.listen(listen_config, Some(2048));
        net.register_message_channel_native(MESSAGE_SETTINGS, &MESSAGE_CHANNEL_ID)
            .unwrap();
        // Naia registration
        net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
            builder.register::<String>(MESSAGE_SETTINGS).unwrap();
        });

        // Finally, insert the network resource so it can be used by other systems
        commands.insert_resource(net);
    }
    if &args[1] == "client" {
        println!("setuping client");
        let mut net = NetworkResource::new_client(tokio_rt.clone(), task_pool.0.clone());
        let connect_config = ConnectConfig {
            addr: "127.0.0.1:9000",
            udp_addr: Some("127.0.0.1:9001"),
        };

        net.connect(connect_config, Some(2048));
        net.register_message_channel_native(MESSAGE_SETTINGS, &MESSAGE_CHANNEL_ID)
            .unwrap();
        // Naia registration
        net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
            builder.register::<String>(MESSAGE_SETTINGS).unwrap();
        });

        commands.insert_resource(net);
    }
}


