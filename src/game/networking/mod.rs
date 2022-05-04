use bevy::{math::*, prelude::*};
pub mod shared;
use bevy_rapier3d::{prelude::*};

pub mod client;
mod server;

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                RapierConfiguration {
                    //physics_pipeline_active: false,
                    //query_pipeline_active: false,
                    ..Default::default()
                }
            )
            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(client::Client) // add run criteria
            .add_plugin(server::Server) // add run criteria
            .run();
    }
}



