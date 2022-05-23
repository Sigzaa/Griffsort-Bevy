use bevy::prelude::*;
use serde::{Serialize, Deserialize};
#[derive(Component,Serialize, Deserialize, Debug, Copy, Clone)]
pub struct GoInputs {
    // Events
    pub mouse_delta_y: f32,
    pub mouse_delta_x: f32,
    pub jump: i8,
    pub left: i8,
    pub right: i8,
    pub forward: i8,
    pub back: i8,
    pub a_1: i8,
    pub a_2: i8,
    pub a_3: i8,
    pub a_4: i8,
    pub a_5: i8,
    pub a_6: i8,
    pub sprint: i8,
    pub fire: i8,
}
impl Default for GoInputs {
    fn default() -> Self {
        Self {
            mouse_delta_x: 0.,
            mouse_delta_y: 0.,
            jump: 0,
            left: 0,
            right: 0,
            forward: 0,
            back: 0,
            a_1: 0,
            a_2: 0,
            a_3: 0,
            a_4: 0,
            a_5: 0,
            a_6: 0,
            sprint: 0,
            fire: 0,
        }
    }
}