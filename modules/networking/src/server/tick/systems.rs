use super::super::resources::*;
//use crate::shared::resources::{filters::*, player_data::*};
use crate::shared::resources::*;
use bevy::prelude::*;
use bevy_simple_networking::Transport;
use core::prelude::*;
use gocha::prelude::*;


pub fn pop_buffer(
    mut q_core: Query<(&mut InputsBuffer, &mut GoInputs, &mut GoRot), With<Core>>,
    tick: ResMut<TickCount>,
) {
    for (mut inp_buf, mut ginp, mut grot) in q_core.iter_mut(){
        let input = inp_buf.0[tick.0];
        if input.is_none(){
            warn!("missed package at {}", tick.0);
            return;
        } else {
            let input = input.unwrap(); 
            *ginp = input.ginp;
            *grot = input.grot;
        }
    }
}

pub fn update_tick(mut s_tick: ResMut<TickCount>) {
    s_tick.0 += 1;
}

pub fn send_sys(
    mut q_core: Query<(&Id, &mut GoInputs, &mut Transform, &mut GoRot), With<Core>>,
    mut transport: ResMut<Transport>,
    con: ResMut<ConnectedList>,
    s_tick: ResMut<TickCount>,
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
