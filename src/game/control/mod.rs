use bevy::prelude::*;
use crate::game::components::*;

use world::*;
pub mod world;
pub mod players;
mod window;
mod console;

pub struct Control;
impl Plugin for Control {
    fn build(&self, app: &mut App) {
        app 
            .add_startup_system(say_hi)
            .add_event::<BindControls>()
            .add_system(window::window_control)
            .add_system(window::cursor_grab_system)
            .add_system(players::control_bind)
            .add_startup_system(spawn_teams)
            .add_startup_system(world::setup_world)
            .add_plugin(console::Console)
            ;
    }
}

fn say_hi(){
    println!("Control Plugin is connected");
}