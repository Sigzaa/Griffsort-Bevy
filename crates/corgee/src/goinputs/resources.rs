use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Default, Component, Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Reflect)]
pub struct GoRot {
    pub x: Quat,
    pub y: Quat,
    pub z: Quat,
}
impl GoRot{
    pub fn default() -> Self{
        Self{
            x: Quat::from_rotation_x(0.),
            y: Quat::from_rotation_y(0.),
            z: Quat::from_rotation_z(0.),
        }
    }
    pub fn new(x: f32, y: f32, z: f32) -> Self{
        Self{
            x: Quat::from_rotation_x(x),
            y: Quat::from_rotation_y(y),
            z: Quat::from_rotation_z(z),
        }
    }
}


#[derive(Component, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Reflect)]
pub struct GoInputs {
    pub mouse_delta_y: f32,
    pub mouse_delta_x: f32,
    pub movement: Vec2,
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
            movement: Vec2::new(0.,0.),
            jump: 0,
            forward: 0,
            left: 0,
            right: 0,
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

pub struct MyGamepad(pub Gamepad);

#[derive(Default)]
pub struct IsGoInActive(pub bool);

#[derive(Default)]
pub struct Sensitivity(pub f32);

#[derive(Component, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum Rot{
    Synced,
    Own(Quat)
}

impl Default for Rot {
    fn default() -> Self { Rot::Synced }
}