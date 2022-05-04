use bevy::window::PresentMode;
use bevy::prelude::*;
use game::Game;
mod game;

//use crate::bevy_console::*;
fn main() {

    App::new()
    //
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


