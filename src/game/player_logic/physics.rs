use crate::game::components::{filters::*, player_data::*, *};
use bevy::{input::mouse::MouseMotion, prelude::*};
use std::env;
pub fn gravity(
    time: Res<Time>,
    mut player: Query<(&mut Transform, &mut VerticalVelocity), With<Core>>,
) {
    let args: Vec<String> = env::args().collect();
    if &args[1] != "server" { return; }
    return;
    for (mut transform, mut velocity) in player.iter_mut() {
        let height = transform.translation[1];
        if velocity.0 > 0. || height > 0.5 {
            let delta = time.delta_seconds();
            velocity.0 -= delta * 40.;
            transform.translation += delta * Vec3::new(0., velocity.0, 0.);
        } else {
            velocity.0 = 0.0;
        }
        if height < 0.35 {
            velocity.0 = 3.0;
        }
    }
}

pub fn movement(
    time: Res<Time>,
    mut q_cores: Query<
        (
            &mut Transform,
            &mut VerticalVelocity,
            &JumpValue,
            &Speed,
            &mut Control,
            Option<&Selected>,
        ),
        (With<Core>, Without<ThreeDCam>),
    >,
    mut q_camera: Query<&mut Transform, With<ThreeDCam>>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    let args: Vec<String> = env::args().collect();
    if &args[1] != "server" || true { return; }
    return;
    let mut camera = q_camera.single_mut();
    for (mut transform, mut velocity, jump_value, hor_velocity, mut ctrl, selected) in
        q_cores.iter_mut()
    {
        let mut direction = Vec3::ZERO;

        let vec = transform.local_x();
        let z = vec[0];
        let x = vec[2];
        if ctrl.back {
            direction.z += z;
            direction.x -= x;
        }
        if ctrl.forward {
            direction.z -= z;
            direction.x += x;
        }
        if ctrl.left {
            direction.z -= x;
            direction.x -= z;
        }
        if ctrl.right {
            direction.z += x;
            direction.x += z;
        }
        let mut coef = hor_velocity.0;
        if ctrl.shift {
            coef *= 1.4;
        }
        if ctrl.jump && velocity.0 == 0. {
            velocity.0 = jump_value.0;
        }
        /*if let Some(_selected) = selected { // For minimisation mouse latency it reads mouse event directly from input ev.
            ctrl.delta_x = 0.;
            ctrl.delta_y = 0.;
            for ev in motion_evr.iter() {
                ctrl.delta_x = -ev.delta.x;
                ctrl.delta_y = -ev.delta.y;
                //println!("{},{}", ctrl.delta_x, ctrl.delta_y);
                transform.rotation *= Quat::from_rotation_y(-ev.delta.x * SENSITIVITY);
                camera.rotation *= Quat::from_rotation_x(-ev.delta.y * SENSITIVITY);
            }
        } else {
            //println!("{},{}", ctrl.delta_x, ctrl.delta_y);
            
            //camera.rotation *= Quat::from_rotation_x(ctrl.delta_y * SENSITIVITY);
        }
        */
        //transform.rotation *= Quat::from_rotation_y(ctrl.delta_x * SENSITIVITY * 5.);
        ctrl.delta_x = 0.;
        ctrl.delta_y = 0.;
        transform.translation += time.delta_seconds() * coef * direction;
    }
}

pub fn head_movement_system(
    mut motion_evr: EventReader<MouseMotion>,
    q_parent: Query<(&mut Transform, &Control, &HeadRotation, &Children), (With<Core>, Without<CustomHeadMovement>)>,
    mut q_child: Query<&mut Transform, Without<Core>>,
) {
    let args: Vec<String> = env::args().collect();
    //if &args[1] != "server" { return; }
    for (_transform, ctrl, head_rotation, children) in q_parent.iter() {
        for &child in children.iter() {
            
                let mut transform = q_child.get_mut(child).unwrap();

                transform.rotation = head_rotation.0;
                
            
        }
    }
}
