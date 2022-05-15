pub mod shared;
pub mod client;
mod server;

use crate::game::components::*;
use shared::{resources::*, systems::*};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use bevy_snap::*;



struct SpecialStagingPlugin {
    schedule: Schedule,
}

impl SpecialStagingPlugin {
    pub fn new(schedule: Schedule) -> Self {
        Self { schedule }
    }
}

impl SpecialStagingPlugin {
    fn build(self, app: &mut App) {
        app.add_stage_before(
            CoreStage::Update,
            "special_staging_plugin_stage",
            SpecialStage::new(self.schedule),
        );
    }
}

struct SpecialStage {
    schedule: Schedule,
}

impl SpecialStage {
    pub fn new(schedule: Schedule) -> Self {
        Self { schedule }
    }
}

impl Stage for SpecialStage {
    fn run(&mut self, world: &mut World) {
        self.schedule.run_once(world);
    }
}

struct FrameCount(u32);

pub struct Networking;
impl Plugin for Networking {
    fn build(&self, mut app: &mut App) {

        SpecialStagingPlugin::new(
            Schedule::default()
                .with_stage(
                    PhysicsStages::SyncBackend,
                    SystemStage::parallel()
                    .with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::SyncBackend),
                    ),
                )
                .with_stage_after(
                    PhysicsStages::SyncBackend,
                    PhysicsStages::StepSimulation,
                    SystemStage::parallel()
                        .with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
                            PhysicsStages::StepSimulation,
                        )),
                )
                .with_stage_after(
                    PhysicsStages::StepSimulation,
                    PhysicsStages::Writeback,
                    SystemStage::parallel()
                    .with_system_set(
                        RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsStages::Writeback),
                    ),
                ),
        )
        .build(&mut app);
    
        // Be sure to setup all four stages
        // app.add_stage_before(
        //     CoreStage::Last,
        //     PhysicsStages::DetectDespawn,
        //     SystemStage::parallel().with_system_set(RapierPhysicsPlugin::<NoUserData>::get_systems(
        //         PhysicsStages::DetectDespawn,
        //     )),
        // );

    
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default().with_default_system_setup(false));
        app
            .insert_resource(
                RapierConfiguration {
                    //physics_pipeline_active: false,
                    //query_pipeline_active: false,
                    timestep_mode: TimestepMode::Fixed{
                        dt: 0.01,
                        substeps: 5,
                    },
                    ..Default::default()
                }
            )
            
            .add_plugin(SnapPlugin::<SnapShot>::default())
            .insert_resource(SaveSlot(Vec::new()))
            .add_system(store_snapshot)
            .add_system(load_snap)

            .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(client::Client) // add run criteria
            .add_plugin(server::Server) // add run criteria
            
            .run();
    }
}



