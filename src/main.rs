use bevy::prelude::*;
//use networking::*;
use core::*;
use gocha::*;

fn main() {
    App::new()
       
        .add_plugins(DefaultPlugins)
        .add_plugin(Gocha)
        .add_plugin(Core)
        //.add_plugin(Networking)
        
        .run();
}