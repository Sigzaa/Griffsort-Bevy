use bevy::{ prelude::*};
use crate::game::components::{ filters::*, player_states::*};

pub fn client_moving(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<&mut Control, With<Selected>>,
    

) {
        
    

    for mut ctrl in q_selected.iter_mut() {
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
        if input.just_pressed(KeyCode::Space){
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
        if input.just_released(KeyCode::Space){
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
