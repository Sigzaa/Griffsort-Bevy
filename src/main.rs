use bevy::prelude::*;
use game::Game;
mod game;

//use crate::bevy_console::*;
fn main() {
    App::new()
    //
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

