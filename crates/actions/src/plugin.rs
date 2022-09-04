use crate::update_inputs;

use super::example::example_ron;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::path::Path;
use bevy::prelude::*;
use serde::Deserialize;
use super::resources::*;
use super::systems::collect_actions;
use std::fs;


impl<Keys, Sel> Plugin for ActionsPlugin <Keys, Sel>

where for<'de> Keys: Eq + std::hash::Hash + Send + Deserialize<'de> + Sync + Clone + Debug + 'static, Sel: Component
{
    fn build(&self, app: &mut App) 
    {
        let mut bindings = Keybindings::<Keys>::default();

        let is_config_exist = Path::new(self.config_path).is_file();
        let is_default_exist = Path::new(self.default_path).is_file();

        match (is_config_exist, is_default_exist)
        {
            // There is no config and default
            (false, false) =>
            {
                // Creating default file if not exists.
                fs::write(self.default_path, example_ron()).unwrap();
                panic!("You have to fill default config file ({}) first", self.default_path);
            },

            //There is default, but there is no config
            (false, true) => 
            {
                fs::copy(self.default_path, self.config_path).unwrap();
            }

            // There is config and default
            (true, true) =>
            {
                let config_str = fs::read_to_string(self.config_path).unwrap();
                bindings = ron::from_str(&config_str).unwrap();
            }

            // Else
            _ => 
            {
                fs::remove_file(self.default_path).unwrap();
                fs::remove_file(self.config_path).unwrap();

                panic!("Keybindings files error, just run programm again");
            }
        }

        app
        .insert_resource(bindings)
        .add_system(collect_actions::<Sel, Keys>.after(update_inputs::<Sel, Keys>));
        


    }
}

pub struct ActionsPlugin<Keys: Eq + std::hash::Hash, Sel: Component>{
    _phantom_data: PhantomData<Keys>,
    _phantom_data2: PhantomData<Sel>,
    config_path: &'static str,
    default_path: &'static str,
}

impl<Keys: Eq + std::hash::Hash, Sel: Component> ActionsPlugin<Keys, Sel>{
    pub fn new(config_path: &'static str, default_path: &'static str) -> Self{
        Self { _phantom_data: PhantomData::default(), _phantom_data2: PhantomData::default(), config_path, default_path }
    }
}

