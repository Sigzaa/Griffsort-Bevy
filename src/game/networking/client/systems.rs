use super::components::*;
use crate::game::components::{filters::*, player_data::*};
use crate::game::networking::shared::additional::*;
use bevy::ecs::schedule::{ShouldRun, ShouldRun::*};
use bevy::{ prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_simple_networking::Transport;

pub fn send_message(
    time: Res<Time>,
    mut transport: ResMut<Transport>,
    mut q_selected: Query<(&Id, &mut Control, &mut Transform, &mut HeadRotation), With<Selected>>,
    serv_addr: ResMut<ServerAddr>,
    my_id: ResMut<MyId>,
    mut inp_his: ResMut<InputHistory>,
    tick_counter: ResMut<Tick>,
) {
    for (_id, ctrl, transform, head_rotation) in q_selected.iter_mut() {
        //
        //println!("send");
        let msg = msg_structure {
            ctrl: ctrl.clone(),
            rotation: transform.rotation,
            head_rotation: head_rotation.0,
            id: my_id.0,
            tick: tick_counter.0,
        }
        .pack();

        // msg_pack : msg_struct -> string to send.
        // msg_unpack : received string -> msg_struct.

        //let msg = msg_s.pack(); // Can also use msg_pack(msg_s)

        // Filling the history buffer. AKA Snapshot
        inp_his.0.push(HisPack {
            tick: tick_counter.0,
            input: ctrl.clone(),
            delta_seconds: time.delta_seconds(), // Remove it
            rotation: transform.rotation,
            translation: transform.translation, // Uses for less calculation
            st_tr: Vec3::new(0., 0., 0.),
        });
        // Limiting buffer.
        if inp_his.0.len() > 10000 {
            inp_his.0.remove(0); // !Change to pop or something. Or prevent pushing to the his.
        }
        if serv_addr.0.len() >= 1 {
            // Check for avaliable server in another place
            // Sending via UDP.
            transport.send("main", serv_addr.0[0], &msg);
        }
    }
}

pub fn predict_sys(
    mut q_core: Query<(&mut Control, &mut Velocity), With<Core>>,
    mut is_started: ResMut<IsStarted>, // Will be replaced with reconsiliation system
) {
    //println!("tick {}:", tick_counter.0);
    //println!("predict");
    // println!();
    for (ctrl, mut rb_velocity) in q_core.iter_mut() {
        // Simulating -->
        let mut rb_vel = Vec3::ZERO;

        // Uses to sync initial conditions
        // Remove this after creating reconciliation system -->
        if !is_started.0 && ctrl.velocity == Vec3::ZERO {
        } else {
            is_started.0 = true;
            rb_vel = Vec3::new(0., rb_velocity.linvel[1], 0.);
        } //<--
          //println!("{}{}", ctrl.velocity, rb_vel);
        rb_velocity.linvel = (ctrl.velocity + rb_vel).into();
        //println!("pos: {}", rb_position.position.translation);

        //println!("rot: {} {}", transform.rotation, ctrl.forward);
        //println!("pos: {:?} + {}", rb_position.position.translation, ctrl.velocity);
    }
}

pub fn update_tick(mut s_tick: ResMut<Tick>) {
    s_tick.0 += 1;
    //println!("tick end");
    //println!();
    //println!("server tick end: {}", s_tick.0);
}


pub fn tick_more_then_zero(tick_counter: Res<Tick>, inp_buf: Res<InpBuf>) -> ShouldRun {
    if tick_counter.0 > 0 && inp_buf.tick > 0 {
        return ShouldRun::Yes;
    }
    ShouldRun::No
}
#[allow(dead_code)]
pub fn rollback(is_rollback: Res<IsRollback>) -> ShouldRun {
    if is_rollback.0 {
        return Yes;
    } else {
        return No;
    }
}

pub fn prepare_rollback(iter: Res<iter_count>, mut is_rollback: ResMut<IsRollback>) {
    if iter.0 > 0 {
        is_rollback.0 = true;
    } else {
        is_rollback.0 = false;
    }
}
pub fn check_for_desync_sys(
    inp_his: ResMut<InputHistory>,
    tick_counter: Res<Tick>,
    inp_buf: Res<InpBuf>,
    mut iter: ResMut<iter_count>,
) {
    let tick = inp_buf.tick as usize;

    let client_tick = inp_his.0[tick].tick as usize;
    let client_pos = inp_his.0[2 * tick - client_tick].translation;
    let client_tick = inp_his.0[2 * tick - client_tick].tick as usize;

    let server_pos = inp_buf.pos;
    if client_pos != server_pos {
        // Time travelling ->
        //println!("desync on {}: server: {} client on {}: {}, w: {}", tick ,server_pos, client_tick, client_pos, client_forward );
        iter.0 = tick_counter.0 - client_tick as i32;
    }
}
