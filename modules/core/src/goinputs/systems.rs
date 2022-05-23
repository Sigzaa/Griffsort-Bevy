use bevy::{input::mouse::MouseMotion, prelude::*};
use crate::prelude::{*, filters::*};

pub fn collect_inputs(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<&mut GoInputs, With<Selected>>,
    grabbed_flag: Res<GrabbedCursor>,
) {
    //println!("collinputs");
    if !grabbed_flag.0 {
        return;
    }
    for mut ginp in q_selected.iter_mut() {
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
