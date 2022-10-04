use super::systems::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

// Resource should be inserted manually.
// Add plugin after inserting resource.
impl<T: for<'de> Deserialize<'de> + Default + Sync + Send + 'static + Serialize> Plugin
    for Synx<T>
{
    fn build(&self, app: &mut App) {
        app
            // Inserting Synx as a resource to access "path" from systems.
            .insert_resource(Synx::<T>::new(self.path))
            // Will panic if resource is not existing.
            .add_startup_system(load_config::<T>)
            .add_system(watch_for_changes::<T>);
    }
}

pub struct Synx<T: Default> {
    pub(crate) path: &'static str,
    _phantom_data: PhantomData<T>,
}

impl<T: Default> Synx<T> {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            _phantom_data: PhantomData,
        }
    }
}

// struct Path<T: Default>{
//     path: &'static str,
//     _phantom_data: PhantomData<T>
// }

// impl <T: Default> Path <T>{
//     fn new(path: &'static str) -> Self{

//     }
// }
