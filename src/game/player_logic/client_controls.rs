use crate::game::components::{filters::*, player_data::*, SENSITIVITY, GrabbedCursor};
use bevy::{input::mouse::MouseMotion, prelude::*};

pub fn client_events(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<(&mut Control, &mut Transform, &mut HeadRotation), With<Selected>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut q_camera: Query<&mut Transform, (With<ThreeDCam>, Without<Selected>)>,
    grabbed_flag: Res<GrabbedCursor>
) {
    if !grabbed_flag.0{
        return;
    }
    for (mut ctrl, mut transform, mut head_rotation) in q_selected.iter_mut() {
        for ev in motion_evr.iter() {
            for mut camera in q_camera.iter_mut() {
                transform.rotation *= Quat::from_rotation_y(-ev.delta.x * SENSITIVITY);
                camera.rotation *= Quat::from_rotation_x(-ev.delta.y * SENSITIVITY);
                head_rotation.0 = camera.rotation;
            }
        }
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
