use game::components::Config;
use bevy::prelude::*;
use game::Game;
use std::env;
use std::fs;
mod game;

//use crate::bevy_console::*;
fn main() {
    App::new()
    //
    .add_startup_system(main_setup)
    .insert_resource(WindowDescriptor {
        title: "Moba".to_string(),
        width: 920.,
        height: 700.,
        vsync: false,
        //mode: bevy::window::WindowMode::Fullscreen,
        ..Default::default()
    })
    //.add_plugin(ConsolePlugin)
    .add_plugins(DefaultPlugins)
    .add_plugin(Game)

    
    .run();
}

fn main_setup(
    mut commands: Commands,
){  
    // Server/Client
    let mode = fs::read_to_string("./mode.txt")
    .expect("Something went wrong reading the file");
    //Server Address
    let address = fs::read_to_string("./addr.txt")
    .expect("Something went wrong reading the file");
    //Inserting global Resources
    commands.insert_resource(Config{
        mode: mode,
        address: address,
    });
}

