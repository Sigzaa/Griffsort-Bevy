use crate::{ClientConfig, Config, RonAssetPlugin};
use bevy::{asset::Asset, prelude::*, reflect::TypeUuid};
use iyes_loopless::prelude::IntoConditionalSystem;
use serde::Deserialize;
use std::marker::PhantomData;

pub struct HotResource<A> {
    file_path: &'static str,
    _marker: PhantomData<A>,
}

impl<A> HotResource<A>
where
    for<'de> A: Deserialize<'de> + Asset,
{
    pub fn new(file_path: &'static str) -> Self {
        Self {
            file_path,
            _marker: PhantomData,
        }
    }


}

fn load_config<C: Default + Asset + Clone>(mut commands: Commands, asset_server: Res<AssetServer>) 
{
    asset_server.watch_for_changes().unwrap();
    let handle: Handle<C> = asset_server.load("../config/character-config/soul.ron");
    commands.insert_resource(handle);
    println!("we are here");
}
fn apply_config<C: Default + Asset + Clone>(
    handle: Res<Handle<C>>,
    mut config: ResMut<Assets<C>>,
    mut keycode_map: ResMut<C>,
){
    println!("ыусщтв");
    // if let Some(file_map) = config.get_mut(&handle){
    //     *keycode_map = file_map.clone();
    // } 
    
}

impl<A> Plugin for HotResource<A>
where
    for<'de> A: Deserialize<'de> + Asset + Default + Clone,
{
    fn build(&self, app: &mut App) {
        app
        .insert_resource(A::default())
        .add_startup_system(load_config::<A>)
        .add_system(apply_config::<A>.run_if_resource_exists::<Assets<A>>())
        ;
    }
}
