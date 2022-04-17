use bevy::{ prelude::* };

use control::Control;
use characters::Characters;
mod control;
mod characters;
mod player_logic;
mod rapier;
mod networking;
pub mod components;

mod ui;

pub struct Game;
impl Plugin for Game {
    fn build(&self, app: &mut App) {
        app
            .add_system(new_frame)
            .add_startup_system(say_hi)
            //.add_startup_system(rapier::rapier_entry)
            .add_plugin(ui::Ui)
            .add_plugin(Control)
            .add_plugin(Characters)
            //.insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
            .add_plugin(player_logic::Logic)
            .add_plugin(networking::Networking)
            
            
            //.add_plugin(characters::soul::Soul)
            ;
    }
}


fn say_hi(){
    println!("Moba is online");
}

fn new_frame(){
    //println!();
    //println!("new frame");
}