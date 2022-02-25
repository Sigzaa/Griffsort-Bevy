use crate::game::components::{filters::*, player_states::*, *};
use bevy::prelude::*;
use bootleg_networking::*;
use std::env;
 

const MESSAGE_CHANNEL_ID: MessageChannelID = MessageChannelID::new(0);

pub fn send(
    mut net: ResMut<NetworkResource>,
    mut q_cores: Query<(&Control, &Id, &Transform), (With<Core>, Without<ThreeDCam>)>,
) {
    let args: Vec<String> = env::args().collect();
    if &args[1] == "server" {
        for (ctrl, id, transform) in q_cores.iter_mut() {
            let axis = transform.rotation.to_scaled_axis();
            let message = format!(
                "{} {} {} {} {} {} {}",
                id.0,
                transform.translation[0],
                transform.translation[1],
                transform.translation[2],
                axis[0],
                axis[1],
                axis[2],
            );

            net.broadcast_message(&message, &MESSAGE_CHANNEL_ID)
                .unwrap();
        }

    }
}
pub fn receive(
    mut net: ResMut<NetworkResource>,
    mut q_cores: Query<(&mut Control, &Id), (With<Core>, Without<ThreeDCam>)>,
) {
    let args: Vec<String> = env::args().collect();

    if &args[1] == "server" {
        let messages = net.view_messages::<String>(&MESSAGE_CHANNEL_ID).unwrap();

        for (_handle, message) in messages.iter() {
            
            let input: Vec<f32> = message
            .split_whitespace()
            .map(|s| s.parse().expect("parse error"))
            .collect();

            println!("input: {:?}", message);

            for (mut ctrl, id) in
                q_cores.iter_mut()
            {
                
                if id.0 == input[0] as i32{
                    ctrl.delta_x = input[1];
                    ctrl.delta_y = input[2];
                    ctrl.forward = boo(input[3]);
                    ctrl.left = boo(input[4]); 
                    ctrl.right = boo(input[5]);
                    ctrl.back = boo(input[6]); 
                    ctrl.q = boo(input[7]);
                    ctrl.lmb = boo(input[8]); 
                    ctrl.rmb = boo(input[9]);
                    ctrl.jump = boo(input[10]);
                    ctrl.shift = boo(input[11]); 
                    ctrl.e = boo(input[12]);
                    
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
