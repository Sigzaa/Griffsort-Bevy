use crate::game::components::{filters::*, player_data::*, *};
use bevy::{core::FixedTimestep, ecs::schedule::ShouldRun, input::mouse::MouseMotion, prelude::*};
use std::{env, str, net::{ UdpSocket }};
use components::*;
pub mod components;
mod connection_handler;
mod tick;
use tick::*;
use super::shared::a_list::AList;
use bevy_simple_networking::{ ServerPlugin };
use bevy_rapier3d::{dynamics::*, prelude::*};
use super::shared::additional::*;

use bevy::{ app::ScheduleRunnerSettings };
use priority_queue::PriorityQueue;

use std::{ time::Duration};

fn run_if_started(
    mut q_core: Query<
        (
            &Id,
            &mut Control,
            &mut Transform,
            &mut HeadRotation,
            &mut Velocity,
        ),
        With<Selected>,
    >,
    mut is_started: ResMut<IsStarted>,
) -> ShouldRun {
    for (id, mut ctrl, mut transform, mut head_rotation, mut rb_velocity) in
        q_core.iter_mut()
    {
        if !is_started.0 && ctrl.velocity == Vec3::ZERO{
            return ShouldRun::Yes;
        } else {
            is_started.0 = true;
            return ShouldRun::Yes;
        }
        
    }
    ShouldRun::Yes
}

#[derive(Default)]
pub struct IsStarted(pub bool);

pub struct Server;
impl Plugin for Server {
    fn build(&self, app: &mut App) {
        let args: Vec<String> = env::args().collect();
        if &args[1] != "server" {
            return;
        }
        let listen_address: &str = &args[2].to_owned();
        let socket = UdpSocket::bind(listen_address).expect("could not bind socket");
        socket
            .set_nonblocking(true)
            .expect("could not set socket to be nonblocking");
        socket
            .set_read_timeout(Some(Duration::from_secs(5)))
            .expect("could not set read timeout");

        info!("Server now listening on {}", listen_address);

        app 
            .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f32(
                0.05,
            )))
            .insert_resource(socket)
            .insert_resource(Buffer(PriorityQueue::new()))
            .insert_resource(ConnectedList(AList::default()))
            .insert_resource(Timer1(0.))
            .insert_resource(tick::IsStarted(false))
            //.add_plugins(MinimalPlugins)
            //.add_plugin(LogPlugin)
            .add_plugin(ServerPlugin)
            .insert_resource(TickCounter(-50))
            .add_system(connection_handler::handler.label("msg_collect"))
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::steps_per_second(TICKRATE))
                    //.with_system(crate::game::player_logic::shooting::shoot_system.label("addition").after("msg_collect"))
                    .with_system(pop_buffer.label("buffer").after("msg_collect"))
                    .with_system(crate::game::player_logic::client_controls::velocity_vector_sys.label("vector").after("buffer"))
                    
                    .with_system(simulate_sys.label("sim").after("vector"))
                    .with_system(update_tick.label("tick").after("sim"))
                    //.with_system(step_world_system::<NoUserData>.label("world_step").after("tick"))
                    .with_system(send_sys.after("world_step"))
            )
            //.add_system(send_message)
            .run();
    }
}
