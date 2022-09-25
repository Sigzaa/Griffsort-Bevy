use std::marker::PhantomData;

use crate::hud::hp_bar;


use super::resources::*;
use super::systems::{insert_body, insert_physics, insert_other};
use bevy::prelude::{shape::*, *};
use bevy::render::camera::Projection;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use synx::Synx;

pub struct Controller1<T: Component + Plugin, Conf: ConfigProps + Send + Sync + 'static> {
    pub char_type: T,
    pub conf_type: PhantomData<Conf>,
    pub conf_path: &'static str,
}

impl<T: Component + Plugin, Conf: ConfigProps + Send + Sync + 'static> Controller1 <T, Conf> {
    pub fn new(config_path: &'static str, hero: T) -> Self{
        Self { char_type: hero, conf_type: PhantomData::default(), conf_path: config_path }
    }
}

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
            .add_plugin(self.char_type)
            .add_plugin(InspectorPlugin::<Conf>::new())
            .add_plugin(Synx::<Conf>::new(self.conf_path))
            .add_system(insert_body::<T>)
            .add_system(insert_physics::<T>)
            .add_system(insert_other::<T>)
            .add_system(sync_config_res_with_components::<T, Conf>);
    }
}

pub fn sync_config_res_with_components<T: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut query: Query<(&mut MaxHp, &mut MaxJumpHeight), With<T>>,
    conf: ResMut<Conf>,
) {
    for (mut max_hp, mut max_jump_height) in &mut query{
        max_hp.0 = conf.props().max_hp;
        max_jump_height.0 = conf.props().max_jump_height;
    }
}


