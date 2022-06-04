//This file is a place, where you can put bevy components, events etc.

// Constants -->
pub const SENSITIVITY: f32 = 0.002;
pub const RESPAWNGAP: f32 = 9.;
// <--

// Events structs -->
pub struct BindControls(pub i32);
pub struct SpawnCharacter(pub &'static str, pub i32, pub i32); // Character name/code, player_id, team.
pub struct ExtendCharacter(pub bevy::prelude::Entity, pub i32, pub i32); // Entity of existing, player_id, team.
pub struct _DespawnCharacter(pub i16 /* id */);

use bevy::prelude::*;

// Resources -->
#[derive(Default)]
pub struct MyAddr(pub i32);

//pub struct ConnectedList(pub Vec<std::net::SocketAddr>);

pub struct BindedId(pub i32);
#[derive(Default)]
pub struct GrabbedCursor(pub bool);
#[derive(Default)]
pub struct LetMePlay(pub bool); // If true - player can contol character.
#[derive(Default)]
pub struct GameMode(pub i8); // 0 - std, 1 - partial spectator (for died players), 2 - spectator, 3 - invincible.
                             // <--

// Timers and counters -->
#[derive(Component)]
pub struct Timer1(pub f32);
#[derive(Component)]
pub struct Timer2(pub f32, pub f32);
#[derive(Component)]
pub struct Timer3(pub f32, pub f32, pub f32);
#[derive(Component)]
pub struct Counter(pub f32);
// <--

// Filters -->
pub mod Character {
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
        pub max_velocity: Speed,
        pub weight: Weight,
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
                max_jump_height: MaxJump(20.),
                max_velocity: Speed(30.),
            }
        }
    }

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
}


// <--

// Information about players in Bevy entities -->
pub mod player_data {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Component)]
    pub struct Trans {
        pub velocity: Vec3,
        pub head_rotation: Quat,
    }
    impl Default for Trans {
        fn default() -> Self {
            Self {
                velocity: Vec3::ZERO,
                head_rotation: Quat::IDENTITY,
            }
        }
    }

}
// <--

// Information about bullets. It stores in Player entity -->
pub mod bullet_states {
    use bevy::prelude::*;
    #[derive(Component)]
    pub struct Bullet;
    #[derive(Component)]
    pub struct BulletLifeTime(pub i16);
    #[derive(Component)]
    pub struct BulletVelocity(pub i16);
    #[derive(Component)]
    pub struct Dmg(pub i32);
}
// <--

// Temporary garbage -->
#[derive(Component)]
pub struct Spawn {
    pub respawn_coords: Vec3,
}
