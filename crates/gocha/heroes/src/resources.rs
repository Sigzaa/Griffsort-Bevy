use super::plugin::Character;
use core::prelude::Character::*;

pub struct SpawnCharacterRequest(pub i32, pub i32); // Character code, team code.

#[derive(Default)]
pub struct CharPlugin<T: 'static> {
    pub char_type: T,
}
