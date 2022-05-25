use super::shared::{resources::*, systems::*};
use bevy::prelude::*;
use crate::{server::plugin::Server, client::plugin::Client};

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, mut app: &mut App) {

        //app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false));
        app
            // .insert_resource(
            //     RapierConfiguration {
            //         //physics_pipeline_active: false,
            //         //query_pipeline_active: false,
            //         timestep_mode: TimestepMode::Fixed{
            //             dt: 0.01,
            //             substeps: 5,
            //         },
            //         ..Default::default()
            //     }
            // )
            
            .add_plugin(Client) // add run criteria or feature
            .add_plugin(Server) // add run criteria
            
            .run();
    }
}


