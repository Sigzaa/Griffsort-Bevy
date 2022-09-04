use super::resources::MyGamepad;
use crate::*;
use bevy::{
    input::{mouse::MouseMotion, keyboard},
    prelude::{KeyCode, *},
};
use crate::goconfig::resources::InputMap;

pub fn collect_inputs( // Collecting Keyboard inputs
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<&mut GoInputs, With<Selected>>,
    key_map: Res<InputMap>,
) {
    for mut ginp in &mut q_selected
    {
        // Cleaning previous inputs
        *ginp = GoInputs::default();

        let w = input.pressed(key_map.forward);
        let s = input.pressed(key_map.back);

        
        if !(s && w) {
            if s {
                ginp.movement[1] = -1.;
            }
            if w {
                ginp.movement[1] = 1.;
            }
        }

        let a = input.pressed(key_map.left);
        let d = input.pressed(key_map.right);

        if !(a && d) {
            if a {
                ginp.movement[0] = -1.;
            }
            if d {
                ginp.movement[0] = 1.;
            }
        }

        if buttons.pressed(key_map.shoot) {
            ginp.shoot = 1;
        }
        if buttons.pressed(key_map.a_1) {
            ginp.a_1 = 1;
        }
        if input.pressed(key_map.a_2) {
            ginp.a_2 = 1;
        }
        if input.pressed(key_map.esc) {
            ginp.esc = 1;
        }
        if input.pressed(key_map.jump) {
            ginp.jump = 1;
        }
    }
}

pub fn new_collect_inputs<S: Component>( // Works always
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut q_selected: Query<(&mut GoInputsNew<KeyCode>, &mut GoInputsNew<MouseButton>), With<S>>,
){
    for (mut ginp_keyboard, mut ginp_mouse) in &mut q_selected
    {
        for i in keyboard.get_just_pressed()
        {
            ginp_keyboard.extend_just_pressed(*i);
            ginp_keyboard.extend_pressed(*i);
        }
        for i in mouse.get_just_pressed()
        {
            ginp_mouse.extend_just_pressed(*i);
            ginp_mouse.extend_pressed(*i);
        }
    }
}


pub fn update_inputs<S: Component>( // Execute once at the start of the tick before collect_inputs system!!!
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut q_selected: Query<(&mut GoInputsNew<KeyCode>, &mut GoInputsNew<MouseButton>), With<S>>,
){
    for (mut ginp_keyboard, mut ginp_mouse) in &mut q_selected
    {
        ginp_keyboard.clear_just_pressed();
        ginp_mouse.clear_just_pressed();

        for i in keyboard.get_just_released()
        {
            ginp_keyboard.extend_just_released(*i);
            ginp_keyboard.reset(*i);
        }
        for i in mouse.get_just_released()
        {
            ginp_mouse.extend_just_released(*i);
            ginp_mouse.reset(*i);
        }
    }
}
use bevy::ecs::schedule::ShouldRun;

pub fn if_not_server() -> ShouldRun{
    let args: Vec<String> = std::env::args().collect();
    
    let exec_type = &args[1];
    return match exec_type.as_str() {
            "server" => ShouldRun::No,
            _ => ShouldRun::Yes,
    };
}