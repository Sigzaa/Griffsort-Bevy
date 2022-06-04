mod characters;

use bevy::prelude::*;
//use networking::*;
use go_core::*;
use go_character::*;
use characters::CharactersImpl;

fn main() {
    App::new()
       
        .add_plugins(DefaultPlugins)
        .add_plugin(CharController)
        .add_plugin(CharactersImpl)
        .add_plugin(Core)
        //.add_plugin(Networking)
        
        .run();
}

