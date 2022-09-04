use std::fmt::Debug;

use bevy::prelude::*;
use super::resources::*;

// Works always in the background
pub fn collect_actions<Sel: Component, Keys: std::hash::Hash + Eq + Sync + Send + Clone + Debug + 'static>( 
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut q_selected: Query<&mut Actions<Keys>, With<Sel>>,
    bindings: Res<Keybindings<Keys>>,
){

    
    for mut actions in &mut q_selected
    {
        for key in bindings.keyboard_bindings.keys(){
            if keyboard.just_pressed(*key)
            {
                actions.pressed.insert(bindings.keyboard_bindings.get(key).unwrap().clone());
                actions.just_pressed.insert(bindings.keyboard_bindings.get(key).unwrap().clone());
            }
        }

        for key in bindings.mouse_bindings.keys(){
            if mouse.just_pressed(*key)
            {
                
                actions.pressed.insert(bindings.mouse_bindings.get(key).unwrap().clone());
                actions.just_pressed.insert(bindings.mouse_bindings.get(key).unwrap().clone());
            }
        }

    }
}

// Execute once at the start of the tick before collect_inputs system!!!
pub fn update_inputs<Sel: Component, Keys: std::hash::Hash + Eq + Sync + Send + Clone + Debug + 'static>( 
    keyboard: Res<Input<KeyCode>>,
    mouse: Res<Input<MouseButton>>,
    mut q_selected: Query<&mut Actions<Keys>, With<Sel>>,
    bindings: Res<Keybindings<Keys>>,
){
    for mut actions in &mut q_selected
    {
        // Clearing "just_pressed" and "just_released" of previous tick
        actions.just_released.clear();
        actions.just_pressed.clear();

        let new_actions = input_to_actions(&keyboard, &mouse, &bindings);

        // Calculate a difference between "pressed"
        let diff = &actions.pressed - &new_actions.pressed;

        // Swap new_actions and actions "pressed" fields.
        actions.pressed = new_actions.pressed;

        // Push diff to "just_released"
        actions.just_released = diff;
    }
}

// Converts mouse and keyboard inputs to actions with filled "pressed" HashSet. 
// "just_pressed" and "just_released" are ignored.
fn input_to_actions<Keys: std::hash::Hash + Eq + Sync + Send + Clone + Debug + 'static>(
    keyboard: &Res<Input<KeyCode>>,
    mouse: &Res<Input<MouseButton>>,
    bindings: &Res<Keybindings<Keys>>,
) -> Actions<Keys>{

    let mut actions = Actions::<Keys>::new();

    for key in bindings.keyboard_bindings.keys(){
        if keyboard.pressed(*key)
        {
            actions.pressed.insert(bindings.keyboard_bindings.get(key).unwrap().clone());
        }
    }

    for key in bindings.mouse_bindings.keys(){
        if mouse.pressed(*key)
        {
            actions.pressed.insert(bindings.mouse_bindings.get(key).unwrap().clone());
        }
    }

    actions
}
