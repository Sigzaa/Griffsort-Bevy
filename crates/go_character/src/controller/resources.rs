use super::plugin::Character;
use go_core::Character::*;
use bevy::prelude::*;

pub struct SpawnChar(pub &'static str, pub i32, pub i32); // Character code, team code, id.

#[derive(Default)]
pub struct Controller<T: 'static> {
    pub char_type: T,
}

#[derive(Component)]
pub struct ZHead;

#[derive(Component)]
pub struct Grounded;

#[derive(Component)]
pub struct RideHeight(pub f32);

