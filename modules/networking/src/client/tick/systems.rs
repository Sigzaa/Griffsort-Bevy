use crate::shared::resources::*;
use bevy::ecs::schedule::{ShouldRun, ShouldRun::*};
use bevy::{ prelude::*};
use gocha::prelude::*;
use bevy_simple_networking::Transport;
use serde::{Serialize, Deserialize};
use super::super::resources::*;
use core::prelude::*;

pub fn send_message(
    time: Res<Time>,
    mut transport: ResMut<Transport>,
    mut q_selected: Query<(&Id, &mut GoInputs, &mut Transform, &mut GoRot), With<Selected>>,
    serv_addr: ResMut<ServerAddr>,
    my_id: ResMut<MyId>,
    tick_counter: ResMut<TickCount>,
) {
    for (_id, ginp, transform, grot) in q_selected.iter_mut() {
        //
        let msg_des = FromClient{ 
            id: my_id.0,
            tick: tick_counter.0,
            inputs: Inputs{
                ginp: ginp.clone(),
                grot: grot.clone(),
            },
        };
        
        let msg_ser = serde_json::to_string(&msg_des).unwrap();

        if serv_addr.0.len() >= 1 {
            // Check for avaliable server in another place
            // Sending via UDP.
            transport.send("main", serv_addr.0[0], &msg_ser);
        }
    }
}

pub fn update_tick(mut tick: ResMut<TickCount>) {
    tick.0 += 1;
}

pub fn check_for_desync_sys(
    external_buf: ResMut<ServerShots>,
    internal_buf: ResMut<InternalShots>,
) {
    let tick = external_buf.0.highest_tick();
    if external_buf.0[tick] != internal_buf.0[tick]{
        warn!("desync on {}", tick);
    }
    todo!();
}