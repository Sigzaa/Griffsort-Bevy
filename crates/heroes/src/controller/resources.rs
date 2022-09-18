use super::plugin::Character;
use bevy::prelude::*;
use bevy_inspector_egui::*;
use bevy_rapier3d::prelude::Collider;
pub use components::*;
use serde::{Deserialize, Serialize};
pub struct SpawnChar(pub &'static str, pub Team, pub i32);
impl SpawnChar {
    pub fn new(code: &'static str, team: Team, id: i32) -> Self {
        Self(code, team, id)
    }
}

pub struct HeroesConfig {
    pub showray: bool,
    pub sensitivity: f32,
    pub ray_color: Color,
}

impl Default for HeroesConfig {
    fn default() -> Self {
        Self {
            showray: false,
            sensitivity: 1.,
            ray_color: Default::default(),
        }
    }
}

pub trait ConfigProps {
    fn props(&self) -> &Config;
}

#[derive(Inspectable, Reflect, Clone, Deserialize, Serialize, Default)]
pub struct CameraConfig {
    pub shake_ampl: f32,
    pub shake_rate: f32,
    pub roll_angle: f32,
    pub roll_in_time: f32,
    pub roll_out_time: f32,
}

#[derive(Inspectable, Reflect, Clone, Deserialize, Serialize)]
pub struct IntersectionShape {
    pub radius: f32,
    pub toi: f32,
    pub source_distance: f32,
}

#[derive(Inspectable, Reflect, Clone, Deserialize, Serialize)]
pub struct Config {
    pub character_name: String,
    pub max_hp: f32,
    pub max_jump_height: f32,
    pub max_velocity: f32,
    pub weight: f32,
    pub acceleration: f32,

    #[inspectable(collapse)]
    pub intersections_shape: IntersectionShape,
    #[inspectable(collapse)]
    pub pointing_shape: IntersectionShape,

    #[inspectable(collapse)]
    pub camera: CameraConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            character_name: "Zero".to_string(),
            max_hp: 500.,
            max_jump_height: 3.,
            max_velocity: 15.,
            weight: 2.,
            acceleration: 1000.,

            camera: CameraConfig {
                shake_ampl: 0.03,
                shake_rate: 15.,
                ..Default::default()
            },
            intersections_shape: IntersectionShape {
                radius: 2.,
                toi: 5.,
                source_distance: 1.,
            },
            pointing_shape: IntersectionShape {
                radius: 2.,
                toi: 5.,
                source_distance: 1.,
            },
        }
    }
}

#[derive(Bundle, Component, Reflect)]
pub struct States {
    pub id: Id,
    pub team: Team,
    pub hp: Hp,
    pub noclip: NoClip,
}
impl Default for States {
    fn default() -> Self {
        Self {
            id: Id(-1),
            team: Team::Dark,
            hp: Hp(500),
            noclip: NoClip(false),
        }
    }
}
impl States {
    pub fn new_with_id(id: i32) -> Self {
        Self {
            id: Id(id),
            ..States::default()
        }
    }
}

mod components {
    use bevy::prelude::*;
    use bevy_rapier3d::prelude::Toi;
    use serde::Deserialize;

    #[derive(Component)]
    pub struct Hero;

    #[derive(Component)]
    pub struct Selected;

    #[derive(Component)]
    pub struct Alive;

    #[derive(Component)]
    pub struct Dead;

    #[derive(Component, Reflect, Default, Clone, PartialEq)]
    pub enum Team {
        #[default]
        Dark,
        Light,
    }

    #[derive(Component, Default)]
    pub struct PointingOn(pub Vec<Entity>);

    #[derive(Component, Reflect, Default, Clone)]
    pub struct NoClip(pub bool);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug)]
    pub struct Hp(pub i32);

    #[derive(Component, Debug, Reflect, Default, Copy, Clone, Deserialize)]
    pub struct Id(pub i32);

    #[derive(Component, Reflect, Default, Clone, Deserialize)]
    pub struct SelectedId(pub Option<i32>);

    #[derive(Component)]
    pub struct Head;

    #[derive(Component, Deref, DerefMut)]
    pub struct HeadLink(pub Entity);

    #[derive(Component, Deref, DerefMut)]
    pub struct CameraLink(pub Entity);

    #[derive(Default, Component)]
    pub struct SelectedCamera;

    #[derive(Default, Component)]
    pub struct HeroCam;

    #[derive(Component)]
    pub struct Grounded;
}
