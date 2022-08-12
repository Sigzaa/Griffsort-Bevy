use super::plugin::Character;
use corgee::*;
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

#[derive(Component)]
pub struct ShootTimer(pub Timer);
#[derive(Component)]
pub struct IsReadyShoot(pub bool);

#[derive(Component)]
pub struct ETimer(pub f32);

#[derive(Component)]
pub struct QTimer(pub f32);

#[derive(Component)]
pub struct FTimer(pub f32);

#[derive(Component)]
pub struct ShiftTimer(pub f32);

pub const CHARACTER: u32 = 0b11;


