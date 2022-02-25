use bevy::prelude::*;
mod simple_client;
mod simple_server;

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(simple_client::Client)
            .add_plugin(simple_server::Server)
            .run();
    }
}
