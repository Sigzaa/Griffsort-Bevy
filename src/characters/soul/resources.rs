use bevy::prelude::*;
use corgee::*;

pub const PLACE_SHIELD: f32 = 1.;
pub const GET_SHIELD: f32 = 1.;
pub const SHIELD_COOLDOWN: f32 = 1.;

#[derive(Component)]
pub struct PreQTimer(pub f32);
#[derive(Component)]
pub struct ShieldUp(pub bool);
#[derive(Component)]
pub struct ShieldPos(pub Option<Transform>);
pub enum ShieldEvent {
    Up,
    Down,
}

#[derive(Component)]
pub struct QLimiter(pub bool);
#[derive(Component)]
pub struct ShieldFather(pub Entity);
#[derive(Component)]
pub struct CrosshairValue(pub f32);
#[derive(Component)]
pub struct PointingOn {
    pub target: Entity,
    pub hit: Toi,
}
#[derive(Default)]
pub struct T(pub f32);

//Vfx Events
pub struct ShootEv();
