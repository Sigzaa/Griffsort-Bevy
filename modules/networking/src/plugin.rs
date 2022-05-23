use super::shared::{resources::*, systems::*};
use bevy::prelude::*;



// struct SpecialStagingPlugin {
//     schedule: Schedule,
// }

// impl SpecialStagingPlugin {
//     pub fn new(schedule: Schedule) -> Self {
//         Self { schedule }
//     }
// }

// impl SpecialStagingPlugin {
//     fn build(self, app: &mut App) {
//         app.add_stage_before(
//             CoreStage::Update,
//             "special_staging_plugin_stage",
//             SpecialStage::new(self.schedule),
//         );
//     }
// }

// struct SpecialStage {
//     schedule: Schedule,
// }

// impl SpecialStage {
//     pub fn new(schedule: Schedule) -> Self {
//         Self { schedule }
//     }
// }

// impl Stage for SpecialStage {
//     fn run(&mut self, world: &mut World) {
//         self.schedule.run_once(world);
//     }
// }



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

            .add_plugin(super::client::Client) // add run criteria
            .add_plugin(super::server::Server) // add run criteria
            
            .run();
    }
}


