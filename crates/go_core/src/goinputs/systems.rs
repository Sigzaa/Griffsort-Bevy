use bevy::{input::mouse::MouseMotion, prelude::*};
use crate::{*, Character::*};

pub fn collect_inputs(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<&mut GoInputs, With<Selected>>,
) {
    for mut ginp in q_selected.iter_mut() {
        if buttons.pressed(MouseButton::Left) {
            ginp.fire = 1;
        }
        if input.just_pressed(KeyCode::S) {
            ginp.back = 1;
        }
        if input.just_pressed(KeyCode::W) {
            ginp.forward = 1;
        }
        if input.just_pressed(KeyCode::A) {
            ginp.left = 1;
        }
        if input.just_pressed(KeyCode::D) {
            ginp.right = 1;
        }
        if input.just_pressed(KeyCode::Q) {
            ginp.a_1 = 1;
        }
        if input.just_pressed(KeyCode::E) {
            ginp.a_2 = 1;
        }
        if input.just_pressed(KeyCode::LShift) {
            ginp.sprint = 1;
        }
        if input.just_pressed(KeyCode::Space) {
            ginp.jump = 1;
        }

        if buttons.just_released(MouseButton::Left) {
            ginp.fire = 0;
        }
        if input.just_released(KeyCode::S) {
            ginp.back = 0;
        }
        if input.just_released(KeyCode::W) {
            ginp.forward = 0;
        }
        if input.just_released(KeyCode::A) {
            ginp.left = 0;
        }
        if input.just_released(KeyCode::D) {
            ginp.right = 0;
        }
        if input.just_released(KeyCode::LShift) {
            ginp.sprint = 0;
        }
        if input.just_released(KeyCode::Space) {
            ginp.jump = 0;
        }
        if input.just_released(KeyCode::Q) {
            ginp.a_1 = 0;
        }
        if input.just_released(KeyCode::E) {
            ginp.a_2 = 0;
        }
    }
}
