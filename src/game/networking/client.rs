use crate::game::components::{filters::*, player_states::*, *};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use bootleg_networking::*;
use std::env;
use std::sync::Arc;
use std::io::{self, BufRead};   

const MESSAGE_CHANNEL_ID: MessageChannelID = MessageChannelID::new(0);
const MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: MESSAGE_CHANNEL_ID.id,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 256,
    packet_buffer_size: 256,
};


pub fn send(
    mut net: ResMut<NetworkResource>,
    mut q_cores: Query<(&Control, &Id), (With<Selected>, Without<ThreeDCam>)>,
) {
    let args: Vec<String> = env::args().collect();
    if &args[1] == "client" {
        for (ctrl, id) in q_cores.iter_mut() {
            let message = format!(
                "{} {} {} {} {} {} {} {} {} {} {} {} {}",
                id.0,
                ctrl.delta_x,
                ctrl.delta_y,
                i8::from(ctrl.forward),
                i8::from(ctrl.left),
                i8::from(ctrl.right),
                i8::from(ctrl.back),
                i8::from(ctrl.q),
                i8::from(ctrl.lmb),
                i8::from(ctrl.rmb),
                i8::from(ctrl.jump),
                i8::from(ctrl.shift),
                i8::from(ctrl.e),
                
            );

            net.broadcast_message(&message, &MESSAGE_CHANNEL_ID)
                .unwrap();
        }
    }
}
use std::cmp;
pub fn receive(
    
    mut commands: Commands,
    mut net: ResMut<NetworkResource>,
    mut q_cores: Query<(&mut Control, &Id, &mut Transform, Entity), (With<Core>, Without<ThreeDCam>)>,
) {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "client" {
        let messages = net.view_messages::<String>(&MESSAGE_CHANNEL_ID).unwrap();
        let mut flag = true;
        for (_handle, message) in messages.iter() {
            
            let input: Vec<f32> = message
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();
            
            
            
            //println!("input: {:?}", message);

            for (mut ctrl, id, mut transform, ent) in
                q_cores.iter_mut()
            {
                if id.0 == input[0] as i32{
                let input_translation = Vec3::new(input[1], input[2], input[3]);
                let diff = input_translation - transform.translation;
                println!("diff: {}", &diff);
                let max = diff.min_element().abs().max(diff.max_element());
                    if max > 0.3{
                        if flag {
                            println!("back-roll");
                            flag = false;
                        }
                        transform.translation[0] = input[1];
                        transform.translation[1] = input[2];
                        transform.translation[2] = input[3];
                    }
                let vec = Vec3::new(input[4], input[5], input[6]);
                transform.rotation = Quat::from_scaled_axis(vec);

                }

                //println!("{}", message);
                //ctrl.forward = message.starts_with("t");
            }
        }
        
    }
}

fn boo(num: f32) -> bool{
    if num == 0.{
        false
    } else {
        true
    }
}
