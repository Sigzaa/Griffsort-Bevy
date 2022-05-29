use crate::shared::resources::*;
use bevy::ecs::schedule::{ShouldRun, ShouldRun::*};
use bevy::{ prelude::*};
use core::prelude::Character::*;
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

        if !serv_addr.0.is_none(){
            transport.send("main", serv_addr.0.unwrap(), &msg_ser);
        }
    }
}

pub fn update_tick(mut tick: ResMut<TickCount>) {
    tick.0 += 1;
}

pub(crate) fn is_desync(
    external_buf: ResMut<ServerShots>,
    internal_buf: ResMut<InternalShots>,
) {
    let tick = match external_buf.0.last_tick(){
        Some(tick) => tick,
        None => return,
    };

    match (&external_buf.0.get(tick), &internal_buf.0.get(tick)){
        (Ok(ext_content), Ok(int_content)) => {
            if ext_content != int_content{
                warn!("desync on {}", tick);
            }
        },
        _ => (),
    }
    todo!();
}