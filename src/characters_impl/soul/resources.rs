use bevy::prelude::*;

use bevy_inspector_egui::Inspectable;
pub use components::*;
use heroes::{CDProps, ConfigProps, CooldownManager};
use serde::Serialize;

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

#[derive(serde::Deserialize, Serialize, Clone, Inspectable)]
pub struct SoulConfig {
    pub should_render: bool,

    #[inspectable(min = 42.0, max = 100.0)]
    pub time_to_place_shield: f32,
    pub time_to_get_shield: f32,
    pub sprint_duration: f32,

    #[inspectable(label = "Cooldown")]
    pub shield_cd: f32,
    pub sprint_cd: f32,
    pub reload: f32,

    #[inspectable(collapse)]
    pub crosshair: CrosshairConfig,

    pub config: heroes::Config,
}

impl ConfigProps for SoulConfig {
    fn props(&self) -> &heroes::Config {
        &self.config
    }
}

impl Default for SoulConfig {
    fn default() -> Self {
        Self {
            should_render: false,
            time_to_place_shield: 2.,
            time_to_get_shield: 2.,
            sprint_duration: 2.,
            shield_cd: 3.,
            sprint_cd: 3.,
            reload: 2.,
            crosshair: CrosshairConfig {
                to_attack_duration: 0.6,
                to_idle_duration: 0.5,
                to_pointing_duration: 0.6,
            },
            config: Default::default(),
        }
    }
}

mod components {
    use bevy::prelude::*;
    use bevy_inspector_egui::Inspectable;
    
    use serde::Serialize;

    #[derive(serde::Deserialize, Serialize, Clone, Inspectable)]
    pub struct CrosshairConfig {
        pub to_attack_duration: f32,
        pub to_idle_duration: f32,
        pub to_pointing_duration: f32,
    }

    #[derive(Component, Default, serde::Deserialize, Clone)]
    pub struct PlaceShield(pub f32);
    #[derive(Component, Default, serde::Deserialize, Clone)]
    pub struct GetShield(pub f32);

    #[derive(Component, Default)]
    pub struct ShieldUp(pub bool);
    #[derive(Component, Default)]
    pub struct ShieldPos(pub Option<Transform>);

    #[allow(dead_code)] 
    pub enum ShieldEvent {
        Up,
        Down,
    }

    #[derive(Component)]
    pub struct ShieldFather(pub Entity);

    #[derive(Component)]
    pub struct CrosshairValue(pub f32);

    #[derive(Default)]
    pub struct T(pub f32);

    //Vfx Events
    //pub struct ShootVFXEv();
}
