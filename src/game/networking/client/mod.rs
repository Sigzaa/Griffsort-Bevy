use bevy_simple_networking::ClientPlugin;
use priority_queue::PriorityQueue;
use crate::game::components::{filters::*, player_data::*, *};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::{physics::NoUserData};
use components::*;
use std::{
    env,
    net::{SocketAddr, UdpSocket},
};
pub mod components;
mod connection_handler;
mod tick;
mod reconciliation;
use reconciliation::*;

#[derive(Default)]
pub struct IsStarted(pub bool);

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

        app.insert_resource(remote_addr)
            .insert_resource(InputHistory(Vec::new()))
            .insert_resource(InpBuf{pos: Vec3::ZERO, tick: -1})
            .insert_resource(TickCounter(0))
            .insert_resource(ServerAddr(Vec::new()))
            .insert_resource(MyId(-5)) // That means that client has no binded id.
            .insert_resource(socket)
            .insert_resource(tick::IsStarted(false))
            .add_startup_system(setup_client)
            //.add_system(rotate)
            //.add_plugins(MinimalPlugins)
            //.add_plugin(LogPlugin)
            .add_system(connection_handler::handler)
            .add_system(crate::game::player_logic::client_controls::collect_inputs_sys.label("collecting_inputs"))
            
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TICKRATE))
                    
                    
                    
                    .with_system(tick::send_message.label("send_to_server").after("collecting_inputs"))
                    .with_system(
                        crate::game::player_logic::shooting::shoot_system
                            .label("addition")
                            .after("send_to_server"),
                    )
                    
                    .with_system(
                        crate::game::player_logic::client_controls::velocity_vector_sys
                            .label("inputs")
                            .after("addition"),
                    )
                    .with_system(tick::predict_sys.label("sim").after("inputs"))
                    .with_system(tick::update_tick.label("tick").after("sim"))
                    .with_system(step_world_system::<NoUserData>.label("step").after("tick"))
                    .with_system(tick::fill_his_sys.label("fill_his").after("step"))
                    //.with_system(reconcile_sys::<NoUserData>.label("reconcile").after("fill_his"))
                    //.with_system(tick::camera_movement.label("cam").after("step"))
                    
            )
            .add_system(crate::game::player_logic::client_controls::smooth_camera.after("step"))
            .add_plugin(ClientPlugin)
            
            
            //.add_system(send_message)
            .run();
    }
}
fn setup_client(
    mut commands: Commands,
    mut q_selected: Query<&mut Transform, With<Selected>>,
){
    for mut transform in q_selected.iter_mut(){
        let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_X
                | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z
                | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y;

            let rigid_body = RigidBodyBundle {
                //body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
                position: transform.translation.into(),
                velocity: RigidBodyVelocity {
                    linvel: Vec3::new(0.0, 0.0, -2.0).into(),
                    angvel: Vec3::new(0.0, 0.0, 0.0).into(),
                }
                .into(),
                forces: RigidBodyForces {
                    gravity_scale: 2.0,
                    ..Default::default()
                }
                .into(),
                activation: RigidBodyActivation::cannot_sleep().into(),
                ccd: RigidBodyCcd {
                    ccd_enabled: true,
                    ..Default::default()
                }
                .into(),
                mass_properties: locked_dofs.into(),
                ..Default::default()
            };
            let collider = ColliderBundle {
                shape: ColliderShape::ball(0.5).into(),
                material: ColliderMaterial {
                    restitution: 0.,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            };
            
    }
}
