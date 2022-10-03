use std::marker::PhantomData;
use super::resources::*;
use super::systems::{insert_body, insert_rest, insert_physics};
use bevy::prelude::{*};
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use serde::{Deserialize, Serialize};
use synx::Synx;

impl<T, Conf> Plugin for Controller1<T, Conf>
where
    T: Component + Plugin + Copy,
    Conf: Inspectable
        + ConfigProps
        + Default
        + Serialize
        + for<'de> Deserialize<'de>
        + Send
        + Sync
        + 'static,
{
    fn build(&self, app: &mut App) {
        app
            // Adding hero`s struct which implented Plugin
            .add_plugin(self.char_type)

            // Egui config for CharacterConfig
            .add_plugin(InspectorPlugin::<Conf>::new())

            // Sync this config with filesystem
            .add_plugin(Synx::<Conf>::new(self.conf_path))

            // Creating character
            .add_system(insert_body::<T>)
            .add_system(insert_physics::<T>)
            .add_system(insert_rest::<T, Conf>)

            // Syncing config (witch is resource) with accesable static components
            .add_system(sync_config_res_with_components::<T, Conf>);
    }
}

/*
    Fill this if you want to sync egui config resource with actual components
    Hardcoded
    Todo! rework inspector plugin and make config not a resource, make it query of components   

    To sync:
        1. Add component to query.
        2. component.0 = props.value; 
        
*/
pub fn sync_config_res_with_components<T: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut query: Query<
        (
            &mut MaxHp,
            &mut MaxJumpHeight,
            &mut AmmoCapacity,
            &mut FireRate,
        ),
        With<T>,
    >,
    conf: ResMut<Conf>,
) {
    for (mut max_hp, mut max_jump_height, mut ammo_capacity, mut fire_rate) in &mut query
    {
        let props = conf.props();

        max_hp.0 = props.max_hp;
        max_jump_height.0 = props.max_jump_height;
        fire_rate.0 = props.fire_rate;
        ammo_capacity.0 = props.ammo_capacity;

    }
}

pub struct Controller1<T: Component + Plugin, Conf: ConfigProps + Send + Sync + 'static> {
    pub char_type: T,
    pub conf_type: PhantomData<Conf>,
    pub conf_path: &'static str,
}

impl<T: Component + Plugin, Conf: ConfigProps + Send + Sync + 'static> Controller1<T, Conf> {
    pub fn new(config_path: &'static str, hero: T) -> Self {
        Self {
            char_type: hero,
            conf_type: PhantomData::default(),
            conf_path: config_path,
        }
    }
}