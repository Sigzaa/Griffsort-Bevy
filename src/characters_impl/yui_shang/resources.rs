use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;
pub use components::*;
use heroes::{CDProps, ConfigProps, CooldownManager};
use serde::Serialize;

#[derive(Component)]
pub struct ShootCD(pub CDProps);

impl CooldownManager for ShootCD {
    fn pull_props<'a>(&'a mut self) -> &'a mut CDProps {
        &mut self.0
    }
}

#[derive(serde::Deserialize, Serialize, Clone, Inspectable)]
pub struct ShangConfig {
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

impl ConfigProps for ShangConfig {
    fn props(&self) -> &heroes::Config {
        &self.config
    }
}

impl Default for ShangConfig {
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
    use super::*;

    #[derive(serde::Deserialize, Serialize, Clone, Inspectable)]
    pub struct CrosshairConfig {
        pub to_attack_duration: f32,
        pub to_idle_duration: f32,
        pub to_pointing_duration: f32,
    }
}
