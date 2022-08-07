//This file is a place, where you can put bevy components, events etc.

// Events structs -->
pub struct _DespawnCharacter(pub i16 /* id */);

use bevy::prelude::*;

#[derive(Default)]
pub struct LTimer1(pub f32);
#[derive(Default)]
pub struct LTimer2(pub f32);
#[derive(Default)]
pub struct LTimer3(pub f32);
#[derive(Default)]
pub struct LTimer4(pub f32);

//pub struct ConnectedList(pub Vec<std::net::SocketAddr>);
#[derive(Default)]
pub struct GrabbedCursor(pub bool);
#[derive(Default)]
pub struct LetMePlay(pub bool); // If true - player can contol character.
#[derive(Default)]
pub struct GameMode(pub i8); // 0 - std, 1 - partial spectator (for died players), 2 - spectator, 3 - invincible.
                             // <--
// Temporary garbage -->
#[derive(Component)]
pub struct Spawn {
    pub respawn_coords: Vec3,
}
use std::collections::HashMap;


#[derive(Debug, Default)]
pub struct Lobby {
    pub players: HashMap<u64, Option<Entity>>,
}

#[derive(Default)]
pub struct CursorIsHided(pub bool);



pub enum FpsLimit{
    Unlimited,
    Limited(i32)
}