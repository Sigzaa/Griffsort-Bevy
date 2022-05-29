use heroes::*;
use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct DefaultChar;

impl Character<DefaultChar> for DefaultChar { }

impl Plugin for DefaultChar {
    fn build(&self, app: &mut App) {
        app
        //.add_system_to_stage(DefaultChar::movement)
        ;
    }
}
