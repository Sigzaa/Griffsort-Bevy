use bevy::prelude::*;
use crate::game::components::*;
use bevy_rapier3d::prelude::*;

use world::*;
use load_map::*;

pub mod world;
pub mod players;
mod load_map;
mod window;
mod console;

pub struct Control;
impl Plugin for Control {
    fn build(&self, app: &mut App) {
        app 
            .insert_resource(GrabbedCursor(false))
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .add_startup_system(say_hi)
            .add_event::<BindControls>()
            .add_system(window::window_control)
            .add_system(window::cursor_grab_system)
            .add_system(players::control_bind)
            .add_startup_system(world::setup_world)
            .add_startup_system(load_map::load_gltf)
            .add_system(load_map::spawn_gltf_objects)
            .add_startup_system(spawn_teams)
            
            .add_plugin(console::Console)
            ;
    }
}

fn say_hi(){
    println!("Control Plugin is connected");
}