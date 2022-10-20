use bevy::prelude::Color;
use bevy_inspector_egui::Inspectable;
use serde::*;

#[derive(Serialize, Deserialize, Inspectable)]
pub struct ToScreenDebugger{
    pub show_fps: bool,
    pub show_inputs: bool,
    pub show_transform: bool,
}

impl Default for ToScreenDebugger{
    fn default() -> Self {
        ToScreenDebugger {
            show_fps: true,
            show_inputs: false,
            show_transform: false,

        }
    }
}

/*
    Why is this here?
    Cuz it is a resource.
    You can access it from Main

    Why not just in main?
    "gs" at the beginning of the crate name, means GriffSort`s
    So this is hardcoded component with some api`s for main.

    How this crates communicating each other? With Main!

*/
#[derive(Deserialize, Serialize, Inspectable, Default)]
pub struct ShapeDebugger{
    pub heroes_aiming: Prop,
    pub shape_casting: Prop,
    pub intersections_test: Prop,
    pub rb_colliders: Prop,
}
#[derive(Deserialize, Serialize, Inspectable, Default)]
pub struct Prop{
    pub enable: bool,
    pub color: Color
}






#[derive(PartialEq, Eq)]
pub enum OpenTab {
    Debug,
    Heroes,
    Console,
    Config,
    About,
}

pub struct gs_inspectorToggle(pub bool);
impl Default for gs_inspectorToggle {
    fn default() -> gs_inspectorToggle {
        gs_inspectorToggle(false)
    }
}

#[derive(PartialEq)]
pub struct Console {
    pub string: String,
    pub history: Vec<String>,
    pub iter: usize,
}
impl Console {
    pub fn new() -> Console {
        Console {
            string: String::from(""),
            history: Vec::new(),
            iter: 0,
        }
    }
}
#[derive(Default)]
pub struct Update {
    pub progress: f32,
    pub is_update: bool,
}

#[derive(PartialEq)]
pub struct Stats {
    pub fps: bool,
    pub tick: bool,
    pub state_list: bool,
    pub state: bool,
    pub net_buffer: bool,
}
impl Stats {
    pub fn new() -> Stats {
        Stats {
            fps: true,
            tick: false,
            state: false,
            state_list: false,
            net_buffer: false,
        }
    }
}
#[derive(PartialEq)]
pub struct Config {
    pub exit_on_del: bool,
}
impl Config {
    pub fn new() -> Config {
        Config { exit_on_del: true }
    }
}
