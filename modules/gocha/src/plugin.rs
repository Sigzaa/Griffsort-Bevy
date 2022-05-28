use bevy::{ prelude::*};
use crate::shared::{systems::*, resources::*};
use crate::characters::*;

pub struct Gocha;
impl Plugin for Gocha {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedId(None))
            .insert_resource(CharList(Vec::new()))
            .add_system(spawn_char)
            .add_plugin(CharPlugin::<DefaultChar>::new(DefaultChar))
            .add_plugin(CharPlugin::<Soul>::new(Soul))
        ;
    }
}
