use bevy::prelude::*;
use bevy::reflect::TypeUuid;
pub use components::*;
use corgee::*;
use heroes::{CooldownManager, CDProps};

#[derive(Component)]
pub struct ShieldCD(pub CDProps);

impl CooldownManager for ShieldCD {
    fn pull_props<'a>(&'a mut self) -> &'a mut CDProps {
        &mut self.0
    }
}

#[derive(Component)]
pub struct EscCD(pub CDProps);

impl CooldownManager for EscCD {
    fn pull_props<'a>(&'a mut self) -> &'a mut CDProps {
        &mut self.0
    }
}

pub const PLACE_SHIELD: f32 = 1.;
pub const GET_SHIELD: f32 = 1.;
pub const SHIELD_COOLDOWN: f32 = 1.;

#[derive(serde::Deserialize, TypeUuid, Default, Bundle, Clone)]
#[uuid = "013be529-bfeb-48b3-1db0-4b8b381a2c46"]
pub struct SoulConfig {
    // Const values
    time_to_place_shield: PlaceShield,
    time_to_get_shield: GetShield,
    shield_cooldown: ShieldCooldown,
    config: Config,
}
mod components {
    use bevy::prelude::*;
    use corgee::*;

    #[derive(Component, Default, serde::Deserialize, Clone)]
    pub struct PlaceShield(pub f32);
    #[derive(Component, Default, serde::Deserialize, Clone)]
    pub struct GetShield(pub f32);
    #[derive(Component, Default, serde::Deserialize, Clone)]
    pub struct ShieldCooldown(pub f32);
    #[derive(Component, Default)]
    pub struct PreQTimer(pub f32);
    #[derive(Component, Default)]
    pub struct ShieldUp(pub bool);
    #[derive(Component, Default)]
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
}
