use super::components::*;
use crate::game::components::{filters::*, player_data::*};
use crate::game::networking::shared::additional::*;
use crate::game::networking::shared::resources::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_simple_networking::Transport;

use super::super::shared::systems::*;
use crate::game::components::*;
use crate::game::player_logic::client_controls::*;
use bevy::{core::FixedTimestep, prelude::*};
use bevy_rapier3d::plugin::{systems::*, *};

pub fn pop_buffer(
    mut q_core: Query<(&mut Control, &Id, &mut Transform, &mut Trans), With<Core>>,
    mut buf: ResMut<Buffer>,
    s_tick: ResMut<TickCounter>,
) {
    loop {
        // Expecting panic while unwraping.
        if buf.0.is_empty() {
            break;
        }
        // Skipping frame if client`s tick more then server`s.
        // Inversing " < " to " > " because tick counter is inversed. Look at Priority Queue filling in connection_handler.
        let (mut _pack, tick) = buf.0.peek().unwrap();

        if -tick > s_tick.0 {
            break;
        }
        // Shading with pop
        let (pack, tick) = buf.0.pop().unwrap();

        if tick == -s_tick.0 {
            for (mut ctrl, id, mut transform, mut trans) in q_core.iter_mut() {
                if id.0 == pack.id {
                    // Kill me :
                    ctrl.forward = pack.ctrl.forward;
                    ctrl.left = pack.ctrl.left;
                    ctrl.back = pack.ctrl.back;
                    ctrl.right = pack.ctrl.right;
                    ctrl.jump = pack.ctrl.jump;
                    ctrl.shift = pack.ctrl.shift;
                    ctrl.lmb = pack.ctrl.lmb;
                    trans.head_rotation = pack.head_rotation;
                    transform.rotation = pack.rotation;
                    //println!("tick: {}, rot: {}",tick, super::super::quat_pack(0., ctrl.rotation[1], 0., ctrl.rotation[3]));
                }
            }
        }
    }
}

pub fn simulate_sys(
    mut q_core: Query<(&Id, &mut Control, &mut Transform, &mut Trans, &mut Velocity), With<Selected>>,
    s_tick: ResMut<TickCounter>,
    mut is_started: ResMut<IsStarted>, // Will be replaced with reconsiliation system
    mut grav_scale: Query<&mut GravityScale>,
) {
    for (_id, ctrl, mut transform, mut trans, mut rb_velocity) in q_core.iter_mut() {
        // Simulating -->

        //println!("time: {}", st_time - timer.0);

        //println!("tick {}:", s_tick.0);
        //println!("rot: {} {}", transform.rotation, ctrl.forward);

        //println!("pos: {:?} + {}", rb_position.position.translation, ctrl.velocity);
        //println!("vec: {}", ctrl.velocity);

        // transform.translation = rb_position.position.translation.into();
        let mut rb_vel = Vec3::ZERO;
        if !is_started.0 && trans.velocity == Vec3::ZERO {
        } else {
            for mut grav_scale in grav_scale.iter_mut() {
                grav_scale.0 = 1.0;
            }
            is_started.0 = true;
            rb_vel = Vec3::new(0.,rb_velocity.linvel[1],0.);
            rb_velocity.linvel = (trans.velocity + rb_vel).into();
        }
        //println!("pos: {}", transform.translation);
        //

        if s_tick.0 > 70 {
            //std::process::exit(0);
        }
        //rb_velocity.linvel = rb_vel.into();
    }
}

pub fn update_tick(mut s_tick: ResMut<TickCounter>) {
    s_tick.0 += 1;
    //println!("tick end: {}", s_tick.0);
    //println!();
}

pub fn send_sys(
    mut q_core: Query<(&Id, &mut Control, &mut Transform, &mut Trans), With<Core>>,
    mut transport: ResMut<Transport>,
    con: ResMut<ConnectedList>,
    s_tick: ResMut<TickCounter>,
) {
    for (id, _ctrl, transform, _head_rotation) in q_core.iter_mut() {
        let v = transform.translation;
        let msg = format!("{} {} {} {} {}", id.0, s_tick.0, v[0], v[1], v[2]);

        for index in 0..con.0.len() {
            // Worse send_all_clients()
            match con.0.is_empty(index) {
                false => transport.send("main", con.0.get_addr(index).unwrap(), &msg),
                true => {}
            }
        }
    }
}
