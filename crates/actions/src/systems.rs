use std::{fmt::Debug, path::Path, fs};
use super::example::example_ron;
use bevy::prelude::*;
use ron::ser::{PrettyConfig, to_string_pretty};
use super::resources::*;
use serde::{Deserialize, Serialize};

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

pub(crate) fn watch_for_changes<Keys: std::hash::Hash + Eq + Sync + Send + Clone + Debug + 'static + Serialize>(
    bindings: Res<Keybindings<Keys>>,
    path: ResMut<KeybindingsPath>,
){
    if bindings.is_changed() {

        let pretty = PrettyConfig::new()
        .depth_limit(2)
        .separate_tuple_members(true)
        .enumerate_arrays(true);
        let s = to_string_pretty(&*bindings, pretty).expect("Serialization failed");


        fs::write(path.path, s.clone()).unwrap();
    }
}

pub(crate) fn load_bindings< Keys: for<'de> Deserialize<'de> + Eq + std::hash::Hash + Send  + Sync + Clone + Debug + 'static>
(
    mut bindings: ResMut<Keybindings<Keys>>,
    path: ResMut<KeybindingsPath>,
){

        let is_config_exist = Path::new(path.path).is_file();
        let is_default_exist = Path::new(path.default_path).is_file();

        match (is_config_exist, is_default_exist)
        {
            // There is no config and default
            (false, false) =>
            {
                // Creating default file if not exists.
                fs::write(path.default_path, example_ron()).unwrap();
                panic!("You have to fill default config file ({}) first", path.default_path);
            },

            //There is default, but there is no config
            (false, true) => 
            {
                fs::copy(path.default_path, path.path).unwrap();
            }

            // There is config and default
            (true, true) =>
            {
                let config_str = fs::read_to_string(path.path).unwrap();
                *bindings = ron::from_str(&config_str).unwrap();
            }

            // Else
            _ => 
            {
                fs::remove_file(path.default_path).unwrap();
                fs::remove_file(path.path).unwrap();

                panic!("Keybindings files error, just run programm again");
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
