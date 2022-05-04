use game::components::Config;
use bevy::window::PresentMode;
use bevy::prelude::*;
use game::Game;
use std::fs;
mod game;

//use crate::bevy_console::*;
fn main() {

    App::new()
    //
    .add_startup_system(main_setup)
    .insert_resource(WindowDescriptor {
        title: "Moba".to_string(),
        width: 720.,
        height: 500.,
        present_mode: PresentMode::Immediate,
        //mode: bevy::window::WindowMode::BorderlessFullscreen,
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

