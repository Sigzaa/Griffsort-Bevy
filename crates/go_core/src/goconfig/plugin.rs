use crate::goinputs::systems::*;
use bevy::prelude::*;

pub struct GoInputs;
impl Plugin for GoInputs {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "Moba".to_string(),
            width: 720.,
            height: 500.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        });
    }
}
