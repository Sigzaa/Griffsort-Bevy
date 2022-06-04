use super::plugin::Character;
use go_core::prelude::Character::*;
use bevy::prelude::*;

pub struct SpawnChar(pub &'static str, pub i32, pub i32); // Character code, team code, id.

#[derive(Default)]
pub struct Controller<T: 'static> {
    pub char_type: T,
}
