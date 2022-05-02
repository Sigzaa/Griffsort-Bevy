use crate::game::components::{filters::*, player_data::*, *};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_simple_networking::ClientPlugin;
use components::*;
use priority_queue::PriorityQueue;
use std::{
    env,
    net::{SocketAddr, UdpSocket},
};
pub mod components;
mod connection_handler;
mod tick;
use tick::*;
mod reconciliation;
use crate::game::player_logic::client_controls::*;
use bevy::prelude::*;
use bevy_system_graph::*;
use iyes_loopless::prelude::*;
use reconciliation::*;
use std::time::Duration;
use ShouldRun::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, StageLabel)]
struct MyFixedUpdate;

#[derive(Default)]
pub struct IsStarted(pub bool);

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
struct TickStage;
use bevy::reflect::TypeRegistry;
use bevy_snap::*;

#[derive(Default)]
struct MySnap;

// impl SnapType for MySnap {
//     fn add_types(registry: &mut TypeRegistry) {
//         // Register the types you want to be saved and loaded
//         registry.write().register::<Transform>();
//         registry.write().register::<Player>();

//         // Resources are also supported
//         registry.write().register::<Steps>();
//     }
// }

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

        let mut netstage = SystemStage::parallel();
        //netstage.add_system(hi);

        // ... add systems to it ...

        app.insert_resource(remote_addr)
            .insert_resource(InputHistory(Vec::new()))
            .insert_resource(iter_count(0))
            .insert_resource(IsRollback(false))
            .insert_resource(InpBuf {
                pos: Vec3::ZERO,
                tick: -1,
            })
            .insert_resource(TickCounter(0))
            .insert_resource(ServerAddr(Vec::new()))
            .insert_resource(MyId(-5)) // That means that client has no binded id. TODO: Change to Enum
            .insert_resource(socket)
            .insert_resource(tick::IsStarted(false))
            .insert_resource(TPS(Timer::from_seconds(2.0, false)))
            .add_startup_system(setup_client)
            .add_system(connection_handler::handler)
            .add_stage_after(
                CoreStage::PreUpdate,
                "tick",
                SystemStage::single_threaded()
                    .with_run_criteria(FixedTimestep::steps_per_second(TICKRATE)),
            )
            .add_system(collect_inputs_sys.label("collect_inputs"))

            .add_system_to_stage(
                "tick",
                check_for_desync_sys.with_run_criteria(tick_more_then_zero), // -
            )
            .add_system_to_stage("tick", send_message.label("send").after("collect_inputs")) // -

            // -> Add another stage
            .add_system_to_stage("tick", prepare_rollback.label("root").after("send")) // +

            .add_system_to_stage("tick", velocity_vector_sys.label("vel").after("root")) // +
            .add_system_to_stage("tick", predict_sys.label("predict").after("vel")) // +

            .add_system_to_stage("tick", update_tick.label("update_tick").after("predict")) // +
            // .add_system_to_stage(
            //     "tick",
            //     step_world_system::<NoUserData>
            //         .label("step")
            //         .after("update_tick"),
            // ) // and this also to backroll // +
            // <-
            .add_system(smooth_camera.after("step"))
            .add_plugin(ClientPlugin)
            .run();

    }
}

struct TPS(Timer);

#[derive(Default)]
pub struct iter_count(pub i32);

#[derive(Default)]
pub struct IsRollback(pub bool);

#[derive(Default)]
pub struct roll_ticker(pub i32);

fn should_tick(time: Res<Time>, mut timer: ResMut<TPS>, mut iter: ResMut<iter_count>) -> ShouldRun {
    if iter.0 == 0 {
        return ShouldRun::Yes;
    }
    iter.0 -= 1;
    return ShouldRun::YesAndCheckAgain;
}

fn tick_more_then_zero(tick_counter: Res<TickCounter>, mut inp_buf: ResMut<InpBuf>) -> ShouldRun {
    if tick_counter.0 > 0 && inp_buf.tick > 0 {
        return ShouldRun::Yes;
    }
    ShouldRun::No
}
fn rollback(is_rollback: Res<IsRollback>) -> ShouldRun{
    if is_rollback.0{
        return Yes;
    } else {
        return No;
    }

}

fn prepare_rollback(
    mut iter: ResMut<iter_count>,
    mut is_rollback: ResMut<IsRollback>,
){
    if iter.0 > 0{
        is_rollback.0 = true;
    } else {
        is_rollback.0 = false;
    }
}
fn check_for_desync_sys(
    inp_his: ResMut<InputHistory>,
    tick_counter: Res<TickCounter>,
    mut inp_buf: ResMut<InpBuf>,
    mut iter: ResMut<iter_count>,
) {

    let tick = inp_buf.tick as usize;


    let client_tick = inp_his.0[tick].tick as usize;
    let client_pos = inp_his.0[2 * tick - client_tick].translation;
    let client_tick = inp_his.0[2 * tick - client_tick].tick as usize;


    let server_pos = inp_buf.pos;
    if client_pos != server_pos{
        // Time travelling ->
        //println!("desync on {}: server: {} client on {}: {}, w: {}", tick ,server_pos, client_tick, client_pos, client_forward );
        iter.0 = tick_counter.0  - client_tick as i32;
    }
}

fn setup_client(mut commands: Commands, mut q_selected: Query<&mut Transform, With<Selected>>) {
    for mut transform in q_selected.iter_mut() {
        // let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_X
        //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z
        //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y;

        //     let rigid_body = RigidBodyBundle {
        //         //body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
        //         position: transform.translation.into(),
        //         velocity: RigidBodyVelocity {
        //             linvel: Vec3::new(0.0, 0.0, -2.0).into(),
        //             angvel: Vec3::new(0.0, 0.0, 0.0).into(),
        //         }
        //         .into(),
        //         forces: RigidBodyForces {
        //             gravity_scale: 2.0,
        //             ..Default::default()
        //         }
        //         .into(),
        //         activation: RigidBodyActivation::cannot_sleep().into(),
        //         ccd: RigidBodyCcd {
        //             ccd_enabled: true,
        //             ..Default::default()
        //         }
        //         .into(),
        //         mass_properties: locked_dofs.into(),
        //         ..Default::default()
        //     };
        //     let collider = ColliderBundle {
        //         shape: ColliderShape::ball(0.5).into(),
        //         material: ColliderMaterial {
        //             restitution: 0.,
        //             ..Default::default()
        //         }
        //         .into(),
        //         ..Default::default()
        //     };
    }
}
