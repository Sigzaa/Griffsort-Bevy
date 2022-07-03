use bevy::prelude::*;
use bevy::window::PresentMode;

pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        println!("config");
        app.insert_resource(WindowDescriptor {
            title: "griffsort".to_string(),
            width: 920.,
            height: 500.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        });
    }
}
