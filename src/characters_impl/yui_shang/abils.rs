use bevy::{prelude::{Query, With}, transform};
use heroes::CameraLink;

use crate::characters_impl::YuiShang;

use super::*;

pub fn shoot(
    mut q: Query<(&CameraLink, &mut ShootCD, &Actions<Action>), With<YuiShang>>,
    cam_q: Query<&Transform>,
    mut commands: Commands,
    time: Res<Time>
){
    for (cam_link, mut cd, act) in &mut q{

        cd.tick_timers(time.delta_seconds());

        if act.pressed(Action::Shoot) && cd.is_ready(0){
            println!("poof");

            cd.add(-1);
            cd.cooldown(0, 0.5);
            
        }
        if cd.is_empty() || act.pressed(Action::Abil1){
            cd.full(14);
            cd.cooldown(0, 2.);
        }

        let transform = cam_q.get(cam_link.0).unwrap();
        println!("cd: {:?}", cd.0);

    }
}