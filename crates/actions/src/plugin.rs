use crate::systems::{load_bindings, watch_for_changes};
use crate::update_inputs;
use std::fmt::Debug;
use std::marker::PhantomData;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use super::resources::*;
use super::systems::collect_actions;



impl<Keys, Sel> Plugin for ActionsPlugin <Keys, Sel>

where for<'de> Keys: Eq + std::hash::Hash + Send + Deserialize<'de> + Serialize + Sync + Clone + Debug + 'static, Sel: Component
{
    fn build(&self, app: &mut App) 
    {
        let bindings = Keybindings::<Keys>::new();
        let bindings_path = KeybindingsPath::new(self.config_path, self.default_path);

        app
        .insert_resource(bindings)
        .insert_resource(bindings_path)
        .add_startup_system(load_bindings::<Keys>)
        .add_system(watch_for_changes::<Keys>)
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

