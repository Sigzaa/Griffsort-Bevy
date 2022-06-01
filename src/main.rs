use bevy::prelude::*;
//use networking::*;
use go_core::*;
use go_character::*;

fn main() {
    App::new()
       
        .add_plugins(DefaultPlugins)
        .add_plugin(Gocha)
        .add_plugin(Core)
        //.add_plugin(Networking)
        
        .run();
}