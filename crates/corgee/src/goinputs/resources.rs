use std::collections::{HashMap, HashSet};

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
/*
ginp.just_pressed(fire);

ginp.pressed(Shoot);

ginp.just_released(Jump);

collect(key){

    pressed_map.push(keys);

    just_pressed_map.push(keys.just_pressed);
}

pub enum Action{
    Shoot,
    Jump,
    Sprint,
    Abil_1,

}

pub struct GoInputs{
    pressed_map: HashMap<bool, Action>,
    just_pressed_map: HashMap<bool, Action>,
    just_released_map: HashMap<bool, Action>
} 

impl GoInputs{
    pub fn pressed(&self, action: Action) -> bool{
        self.pressed_map.by_key(action)
    }
    pub fn just_pressed(action: Action) -> bool{

    }
    pub fn just_released(action: Action) -> bool{

    }
}



*/
// #[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq)]
// pub enum Action{
//     Shoot,
//     Jump,
//     Sprint,
//     Abil1,

// }


#[derive(Component, Serialize, Deserialize, Clone)]
pub struct GoInputsNew<T: Eq + std::hash::Hash> {
    pub(crate) pressed: HashSet<T>,
    pub(crate) just_pressed: HashSet<T>,
    pub(crate) just_released: HashSet<T>,
}
impl<T: Eq + std::hash::Hash> GoInputsNew <T>{

    pub fn pressed(&self, key: T) -> bool{
        self.pressed.contains(&key)
    }
    pub fn just_pressed(&self, key: T) -> bool{
        self.just_pressed.contains(&key)
    }
    pub fn just_released(&self, key: T) -> bool{
        self.just_released.contains(&key)
    }

    pub(crate) fn clear_just_pressed(&mut self){
        self.just_pressed.clear();
    }
    pub(crate) fn reset(&mut self, key: T){
        self.pressed.remove(&key);
    }
    pub(crate) fn extend_pressed(&mut self, key: T){
        self.pressed.insert(key);
    }
    pub(crate) fn extend_just_pressed(&mut self, key: T){
        self.just_pressed.insert(key);
    }
    pub(crate) fn extend_just_released(&mut self, key: T){
        self.just_released.insert(key);
    }
}
#[derive(Component, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Reflect)]
pub struct GoInputs {
    pub movement: Vec2, // W,A,S,D
    pub jump: i8, // Space
    pub shoot: i8, // Left Mouse Button
    pub a_1: i8, // Right Mouse Button
    pub a_2: i8, // Q
    pub a_3: i8, // F
    pub ult: i8, // E
    pub esc: i8, // Shift
    pub crouch: i8, // Ctrl
}
impl GoInputs{
    pub fn pressed(){
        todo!();
    }
    pub fn just_pressed(){
        todo!();
    }
    pub fn jump(&self) -> bool{
        return match self.jump{
            1 => true,
            _ => false,
        }
    }
    pub fn shoot(&self) -> bool{
        return match self.shoot{
            1 => true,
            _ => false,
        }
    }
    pub fn a_1(&self) -> bool{
        return match self.a_1{
            1 => true,
            _ => false,
        }
    }
    pub fn a_2(&self) -> bool{
        return match self.a_2{
            1 => true,
            _ => false,
        }
    }
    pub fn a_3(&self) -> bool{
        return match self.a_3{
            1 => true,
            _ => false,
        }
    }
    pub fn ult(&self) -> bool{
        return match self.ult{
            1 => true,
            _ => false,
        }
    }
    pub fn esc(&self) -> bool{
        return match self.esc{
            1 => true,
            _ => false,
        }
    }
    pub fn crouch(&self) -> bool{
        return match self.crouch{
            1 => true,
            _ => false,
        }
    }
}
impl Default for GoInputs {
    fn default() -> Self {
        Self {

            movement: Vec2::new(0.,0.),
            jump: 0,
            a_1: 0,
            a_2: 0,
            a_3: 0,
            crouch: 0,
            ult: 0,
            esc: 0,
            shoot: 0,
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