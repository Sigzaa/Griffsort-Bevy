use bevy::prelude::*;

#[derive(Component)]
pub struct ChCore;

#[derive(Component)]
pub struct Selected;

#[derive(Component, Debug)]
pub struct Id(pub i32);

#[derive(Default)]
pub struct SelectedId(pub Option<i32>);

//<--
#[derive(Bundle, Component)]
pub struct Config {
    pub character_name: CharName,
    pub max_hp: MaxHp,
    pub max_jump_height: MaxJump,
    pub max_velocity: MaxSpeed,
    pub weight: Weight,
    pub acceleration: Acceleration,
}
#[derive(Bundle, Component)]
pub struct States {
    pub id: Id,
    pub team: Team,
    pub hp: Hp,
    pub spawn: SpawnCoords,
}
impl Default for States {
    fn default() -> Self {
        Self {
            id: Id(-1),
            team: Team(0),
            hp: Hp(500),
            spawn: SpawnCoords(Vec3::new(2., 30., 15.)),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            character_name: CharName(None),
            max_hp: MaxHp(500),
            weight: Weight(20.),
            max_jump_height: MaxJump(5.),
            max_velocity: MaxSpeed(11.),
            acceleration: Acceleration(500.),
        }
    }
}

#[derive(Component)]
pub struct CharName(pub Option<&'static str>);
#[derive(Component)]
pub struct Team(pub i16);
#[derive(Component)]
pub struct Acceleration(pub f32);
#[derive(Component)]
pub struct Hp(pub i32);
#[derive(Component)]
pub struct MaxHp(pub i32);
#[derive(Component)]
pub struct JumpValue(pub f32);
#[derive(Component)]
pub struct VerticalVelocity(pub f32);
#[derive(Component)]
pub struct MaxSpeed(pub f32);
#[derive(Component)]
pub struct Weight(pub f32);
#[derive(Component)]
pub struct SpawnCoords(pub Vec3);
#[derive(Component)]
pub struct MaxJump(pub f32);

use bevy::prelude::*;
#[derive(Component)]
pub struct CustomGravity;
#[derive(Component)]
pub struct Reconcile;
#[derive(Component)]
pub struct CustomHeadMovement;
#[derive(Default, Component)]
pub struct CharacterCamera;
#[derive(Default, Component)]
pub struct SelectedCamera;
#[derive(Component)]
pub struct Crosshair;
#[derive(Component)]
pub struct Body;
#[derive(Component)]
pub struct Head;

#[derive(Component)]
pub struct Killed {
    pub timer: f32,
}
#[derive(Component)]
pub struct FpsText;
#[derive(Component)]
pub struct HpText;
#[derive(Component)]
pub struct UiCam;
pub struct BindControls(pub i32);
pub struct SpawnCharacter(pub &'static str, pub i32, pub i32); // Character name/code, player_id, team.
pub struct ExtendCharacter(pub bevy::prelude::Entity, pub i32, pub i32); // Entity of existing, player_id, team.