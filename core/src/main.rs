pub mod inputs;
mod shared;

use bevy::prelude::*;
use bevy::window::PresentMode;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Moba".to_string(),
            width: 720.,
            height: 500.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        //.add_plugin(Game)
        .run();
}

pub mod prelude {
    pub use crate::inputs::{resources::*, *};
    pub use crate::shared::resources::*;
}


