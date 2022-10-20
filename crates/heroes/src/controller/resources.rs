use bevy::prelude::*;
pub use bevy_inspector_egui::{widgets::{InspectorQuerySingle, InspectorQuery}, Inspectable, InspectorPlugin, egui, egui::Grid};
pub use bevy_inspector_egui::*;
pub use components::*;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Serialize, Clone, Inspectable)]
pub struct HeroesConfig {
    pub showray: bool,
    pub sensitivity: f32,
    pub ray_color: Color,
    pub debug_rapier: bool,
    pub hp_bar_size: [f32; 2],
}

impl Default for HeroesConfig {
    fn default() -> Self {
        Self {
            hp_bar_size: [150., 20.],
            debug_rapier: false,
            showray: false,
            sensitivity: 1.,
            ray_color: Default::default(),
        }
    }
}

#[derive(Inspectable, Default)]
pub struct DebugAbils {
    hp: InspectorQuerySingle<&'static mut Hp, With<Selected>>,
    ammo: InspectorQuerySingle<&'static mut AmmoCapacity, With<Selected>>,
}

pub trait ConfigProps {
    fn props(&self) -> &Config;
}

#[derive(Inspectable, Reflect, Clone, Deserialize, Serialize, Default, Component)]
pub struct CameraConfig {
    pub shake_ampl: f32,
    pub shake_rate: f32,
    pub roll_angle: f32,
    pub roll_in_time: f32,
    pub roll_out_time: f32,
}

#[derive(Inspectable, Reflect, Clone, Deserialize, Serialize, Component, Default)]
pub struct IntersectionShape {
    #[inspectable(min = 0., max = 4.)]
    pub radius: f32,

    #[inspectable(min = 0., max = 400.)]
    pub toi: f32,

    #[inspectable(min = 0., max = 10.)]
    pub source_distance: f32,
}

#[derive(Inspectable, Reflect, Clone, Deserialize, Serialize, Component)]
pub struct Config {
    pub character_name: String,
    pub max_hp: i32,
    pub max_jump_height: f32,
    pub max_velocity: f32,
    pub weight: f32,
    pub acceleration: f32,
    pub fire_rate: f32,
    pub ammo_capacity: i32,
    pub head_ride_height: f32,

    #[inspectable(collapse)]
    pub intersections_shape: IntersectionShape,
    #[inspectable(collapse)]
    pub pointing_shape: IntersectionShape,
    #[inspectable(collapse)]
    pub pointing_ray_toi: f32,

    #[inspectable(collapse)]
    pub camera: CameraConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            character_name: "".to_string(),
            max_hp: 500,
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
            pointing_ray_toi: 10.,
            fire_rate: 1.4,
            ammo_capacity: 20,
            head_ride_height: 0.7,
        }
    }
}

#[derive(Bundle, Component, Reflect, Default)]
pub struct StaticHeroesBundle {
    pub id: Id,
    pub team: Team,
    pub max_hp: MaxHp,
    pub max_jump_height: MaxJumpHeight,
    pub max_velocity: MaxVelocity,
    pub weight: Weight,
    pub walk_acceleration: WalkAcceleration,
    pub intersections_shape: IntersectionShape,
    pub poinging_ray_toi: RayToi,
    pub fire_rate: FireRate,
    pub ammo_capacity: AmmoCapacity,
    pub head_ride_height: HeadRideHeight,
}
#[derive(Bundle, Component, Reflect, Default)]
pub struct DynamicHeroesBundle {
    pub hp: Hp,
    pub noclip: NoClip,
    pub resists: Resists
}

mod components {
    use bevy::prelude::*;
    //use bevy_inspector_egui::{*, egui::Grid, egui};
    use serde::Deserialize;
    use super::*;

    #[derive(Component, Default, Reflect, Inspectable)]
    pub struct FireRate(pub f32);

    #[derive(Component, Default, Reflect, Inspectable)]
    pub struct CameraShakeIn(pub f32);

    #[derive(Component, Default, Reflect, Inspectable)]
    pub struct HeadRideHeight(pub f32);

    #[derive(Component, Default, Reflect, Inspectable)]
    pub struct AmmoCapacity(pub i32);

    #[derive(Component, Default, Reflect, Inspectable)]
    pub struct Resists{
        common_attack: f32,
        poison: f32
    }

    #[derive(Component)]
    pub struct Hero;

    #[derive(Component)]
    pub struct Selected;

    #[derive(Component)]
    pub struct Alive;

    #[derive(Component)]
    pub struct Dead;

    #[derive(Component, Reflect, Default, Clone, PartialEq, Debug)]
    pub enum Team {
        #[default]
        Dark,
        Light,
    }

    #[derive(Component, Default)]
    pub struct ShapeIntersections(pub Vec<Entity>);

    #[derive(Component, Default)]
    pub struct RayPointingOn(pub Option<(Entity, f32)>);

    #[derive(Component, Reflect, Default, Clone)]
    pub struct NoClip(pub bool);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug, Inspectable)]
    pub struct Hp(pub i32);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug)]
    
    pub struct MaxHp(pub i32);

    impl Inspectable for MaxHp{
        type Attributes = ();

        fn ui(&mut self, ui: &mut bevy_inspector_egui::egui::Ui, options: Self::Attributes, context: &mut bevy_inspector_egui::Context) -> bool {


        let mut changed = false;

            ui.vertical_centered(|ui| {
                Grid::new(context.id()).show(ui, |ui| {
                    ui.add(egui::Slider::new(&mut self.0, 0..=2000));
                    //changed |= self.translation.ui(ui, Default::default(), context);
                    ui.end_row();
                });
            });

        changed
        
        }
        
        
    }

    #[derive(Component, Reflect, Default, Copy, Clone, Debug, Inspectable)]
    pub struct MaxJumpHeight(pub f32);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug, Inspectable)]
    pub struct MaxVelocity(pub f32);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug, Inspectable)]
    pub struct Weight(pub f32);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug, Inspectable)]
    pub struct WalkAcceleration(pub f32);

    #[derive(Component, Reflect, Default, Copy, Clone, Debug, Inspectable)]
    pub struct RayToi(pub f32);

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

    #[derive(Component)]
    pub struct Crosshair;
}
