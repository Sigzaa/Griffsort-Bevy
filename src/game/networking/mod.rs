use bevy::{math::*, prelude::*};
pub mod a_list;
pub mod additional;
use bevy_rapier3d::{prelude::*, physics::TimestepMode};

pub mod client;
mod server;

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                RapierConfiguration {
                    physics_pipeline_active: true,
                    timestep_mode: TimestepMode::FixedTimestep,
                    ..Default::default()
                }
            )
            .add_plugin(client::Client) // add run criteria
            .add_plugin(server::Server) // add run criteria
            .run();
    }
}

pub fn quat_from_vec(v: Vec4) -> Quat{
    Quat::from_xyzw(v[0], v[1], v[2], v[3])
}


