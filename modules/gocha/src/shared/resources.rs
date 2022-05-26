use serde::{Serialize, Deserialize};
use bevy::prelude::*;

#[derive(Default, Component, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct GoRot {
    pub x: Quat,
    pub y: Quat,
    pub z: Quat,
}

#[derive(Component)]
pub struct GoVel(pub Vec3);

#[derive(Component)]
pub struct Core;

#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Selected;

#[derive(Default)]
pub struct SelectedId(pub Option<i32>);

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
pub struct SpawnCharacter(pub &'static str, pub i32); // Character name, team.
//<--
#[derive(Bundle, Component)]
pub struct Config {
    pub character_name: CharName,
    pub max_hp: MaxHp,
    pub max_jump_height: MaxJump,
    pub max_velocity: Speed,
    pub weight: Weight,
}
#[derive(Bundle, Component)]
pub struct States {
    pub character_name: CharName,
    pub id: Id,
    pub team: Team,
    pub hp: Hp,
    pub spawn: SpawnCoords,
}


impl Default for Config {
    fn default() -> Self {
        Self {
            character_name: CharName(None),
            max_hp: MaxHp(500),
            weight: Weight(20.),
            max_jump_height: MaxJump(20.),
            max_velocity: Speed(30.),
        }
    }
}

#[derive(Component, Debug)]
pub struct Id(pub i32);
#[derive(Component)]
pub struct CharName(pub Option<&'static str>);
#[derive(Component)]
pub struct Team(pub i16);
#[derive(Component)]
pub struct Hp(pub i32);
#[derive(Component)]
pub struct MaxHp(pub i32);
#[derive(Component)]
pub struct JumpValue(pub f32);
#[derive(Component)]
pub struct VerticalVelocity(pub f32);
#[derive(Component)]
pub struct Speed(pub f32);
#[derive(Component)]
pub struct Weight(pub f32);
#[derive(Component)]
pub struct SpawnCoords(pub f32);
#[derive(Component)]
pub struct MaxJump(pub f32);