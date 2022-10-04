use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use heroes::*;
use serde::{Deserialize, Serialize};

pub use self::components::*;

#[derive(Component)]
pub struct MarksCD(pub CDProps);

impl CooldownManager for MarksCD {
    fn pull_props<'a>(&'a mut self) -> &'a mut CDProps {
        &mut self.0
    }
}
pub struct RecalkAnglesEv;

pub struct SpawnMarkEv {
    pub amount: usize,
    pub owner: Entity,
}

#[derive(serde::Deserialize, Serialize, Inspectable)]
pub struct JacquelineConfig {
    pub sprint_duration: f32,

    #[inspectable(collapse)]
    pub marks: MarksConfig,

    #[inspectable(label = "Cooldown")]
    pub shield_cd: f32,
    pub sprint_cd: f32,
    pub reload: f32,

    #[inspectable(collapse)]
    pub crosshair: CrosshairConfig,

    pub config: heroes::Config,
}

impl ConfigProps for JacquelineConfig {
    fn props(&self) -> &heroes::Config {
        &self.config
    }
}

impl Default for JacquelineConfig {
    fn default() -> Self {
        Self {
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
            marks: MarksConfig {
                max_amount: 5,
                rotation_offset: 90.,
                distance: 1.,
                animation_delay: 0.5,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Inspectable)]
pub struct MarksConfig {
    #[inspectable(min = 3, max = 15)]
    pub max_amount: usize,
    #[inspectable(min = 0., max = 90.)]
    pub rotation_offset: f32,
    #[inspectable(min = 0.1, max = 2.)]
    pub distance: f32,
    #[inspectable(min = 0., max = 1.)]
    pub animation_delay: f32,
}

mod components {

    use std::collections::HashSet;

    use super::*;

    #[derive(serde::Deserialize, Serialize, Clone, Inspectable)]
    pub struct CrosshairConfig {
        pub to_attack_duration: f32,
        pub to_idle_duration: f32,
        pub to_pointing_duration: f32,
    }

    #[derive(Component)]
    pub struct MarksLinks(pub HashSet<Entity>);

    #[derive(Component)]
    pub enum MarkState {
        // Enemy entity
        Chasing(Entity),

        // Mark close to the enemy
        ReadyToJump(Entity),

        // Is using as a shield
        Shield,

        // Angle
        Idle(f32),
    }

    #[derive(Component)]
    pub struct JumpingTo(pub Entity);

    #[derive(Component)]
    pub struct ShieldState {
        pub link: Option<Entity>,
        pub duration: f32,
    }

    #[derive(Component)]
    pub struct Shield;

    #[derive(Component)]
    pub struct MarkDespawnTimer {
        /// track when the bomb should explode (non-repeating timer)
        pub timer: Timer,
    }
}
