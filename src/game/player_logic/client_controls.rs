use crate::game::components::SPEED;
use crate::game::components::{filters::*, player_data::*, GrabbedCursor};
use bevy::{input::mouse::MouseMotion, prelude::*};
pub fn collect_inputs_sys(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<(&mut Control, &mut Transform, &mut HeadRotation), With<Selected>>,
    _motion_evr: EventReader<MouseMotion>,
    _q_camera: Query<&mut Transform, (With<ThreeDCam>, Without<Selected>)>,
    grabbed_flag: Res<GrabbedCursor>,
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    //println!("collinputs");
    if !grabbed_flag.0 {
        return;
    }
    for (mut ctrl, _transform, _head_rotation) in q_selected.iter_mut() {
        ctrl.delta_x = 0.;
        ctrl.delta_y = 0.;
        if buttons.pressed(MouseButton::Left) {
            ctrl.lmb = true;
        }
        if input.just_pressed(KeyCode::S) {
            ctrl.back = true;
        }
        if input.just_pressed(KeyCode::W) {
            ctrl.forward = true;
        }
        if input.just_pressed(KeyCode::A) {
            ctrl.left = true;
        }
        if input.just_pressed(KeyCode::D) {
            ctrl.right = true;
        }
        if input.just_pressed(KeyCode::Q) {
            ctrl.q = true;
        }
        if input.just_pressed(KeyCode::E) {
            ctrl.e = true;
        }
        if input.just_pressed(KeyCode::LShift) {
            ctrl.shift = true;
        }
        if input.just_pressed(KeyCode::Space) {
            ctrl.jump = true;
        }

        if buttons.just_released(MouseButton::Left) {
            ctrl.lmb = false;
        }
        if input.just_released(KeyCode::S) {
            ctrl.back = false;
        }
        if input.just_released(KeyCode::W) {
            ctrl.forward = false;
        }
        if input.just_released(KeyCode::A) {
            ctrl.left = false;
        }
        if input.just_released(KeyCode::D) {
            ctrl.right = false;
        }
        if input.just_released(KeyCode::LShift) {
            ctrl.shift = false;
        }
        if input.just_released(KeyCode::Space) {
            ctrl.jump = false;
        }
        if input.just_released(KeyCode::Q) {
            ctrl.q = false;
        }
        if input.just_released(KeyCode::E) {
            ctrl.e = false;
        }
    }
}

use crate::game::components::SENSITIVITY;

pub fn smooth_camera(
    mut q_selected: Query<(&mut Control, &mut HeadRotation, &mut Transform), With<Selected>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut q_camera: Query<&mut Transform, (With<ThreeDCam>, Without<Selected>)>,
) {
    for (mut ctrl, mut head_rotation, mut transform) in q_selected.iter_mut() {
        for mut camera in q_camera.iter_mut() {
            for ev in motion_evr.iter() {
                ctrl.delta_x += -ev.delta.x;
                ctrl.delta_y += -ev.delta.y;
                
                transform.rotation *= Quat::from_rotation_y(-ev.delta.x * SENSITIVITY);
                camera.rotation *= Quat::from_rotation_x(-ev.delta.y * SENSITIVITY);
                head_rotation.0 = camera.rotation;
            }
        }
    }
}
pub fn velocity_vector_sys(
    mut query: Query<(&mut Control, &Transform, &mut HeadRotation), With<Core>>,
) {
    for (mut ctrl, transform, _head_rotation) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let vec = transform.local_x();
        let z = vec[0];
        let x = vec[2];

        if ctrl.back {
            direction.z += z;
            direction.x -= x;
        }
        if ctrl.jump {
            direction.y += 0.04;
        }
        if ctrl.e {
            direction.y -= 0.08;
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
        let mut coef = SPEED;
        if ctrl.shift {
            coef *= 3.4;
        }

        ctrl.delta_x = 0.;
        ctrl.delta_y = 0.;
        ctrl.velocity = coef * direction * SPEED;
    }
}
