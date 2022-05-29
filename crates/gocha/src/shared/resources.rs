use serde::{Serialize, Deserialize};
use bevy::prelude::*;
use core::prelude::Character::*;
use heroes::Character;


#[derive(Component)]
pub struct GoVel(pub Vec3);

#[derive(Component)]
pub struct Head;

#[derive(Default)]
pub struct CharList(pub Vec<Entity>);

#[derive(Component)]
pub struct GameMode(pub Mode);

pub enum Mode{
    NoClip,
    Player,
    Invincible
}

#[derive(Component)]
pub struct Killed {
    pub timer: f32,
}


// Events structs -->


