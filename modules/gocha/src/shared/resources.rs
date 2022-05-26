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
pub struct BindedId(pub i32);

#[derive(Component)]
pub struct Killed {
    pub timer: f32,
}

#[derive(Bundle, Component)]
pub struct States {
    pub character_name: CharName,
    pub id: Id,
    pub team: Team,
    pub hp: Hp,
    pub max_hp: MaxHp,
    pub jump_value: JumpValue,
    pub vert_vel: VerticalVelocity,
    pub hor_vel: Speed,
    pub weight: Weight,
}
impl Default for States {
    fn default() -> Self {
        Self {
            character_name: CharName("Not given"),
            team: Team(0),
            hp: Hp(500),
            max_hp: MaxHp(500),
            hor_vel: Speed(100.),
            jump_value: JumpValue(15.),
            id: Id(118),
            vert_vel: VerticalVelocity(0.),
            weight: Weight(20.),
        }
    }
}

#[derive(Component, Debug)]
pub struct Id(pub i32);
#[derive(Component)]
pub struct CharName(pub &'static str);
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