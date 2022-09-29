use bevy::prelude::*;
use colored::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    hash::Hash,
};

// Add reset config

// Watch changes in file

// Watch changes in resource

#[derive(Debug, Deserialize, Serialize)]
pub struct Keybindings<Value> {
    pub mouse_bindings: HashMap<MouseButton, Value>,
    pub keyboard_bindings: HashMap<KeyCode, Value>,
}

impl<Value> Keybindings<Value> {
    pub fn new() -> Self {
        Self {
            keyboard_bindings: HashMap::new(),
            mouse_bindings: HashMap::new(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct KeybindingsPath {
    pub(crate) path: &'static str,
    pub(crate) default_path: &'static str,
}
impl KeybindingsPath {
    pub(crate) fn new(path: &'static str, default_path: &'static str) -> Self {
        Self { path, default_path }
    }
}

#[derive(Debug)]
pub struct IsLocked(pub bool);

#[derive(Component)]
pub struct Actions<A: Eq + Hash + Debug> {
    // Todo
    // Add feature
    pub(crate) _cross: Vec2,

    pub pressed: HashSet<A>,
    pub just_pressed: HashSet<A>,
    pub just_released: HashSet<A>,
}

impl<A: Eq + Hash + Clone + Debug> Actions<A> {
    pub fn cross(&self, left: A, right: A, back: A, forward: A) -> (f32, f32) {
        let hor: f32 = match (self.pressed(left), self.pressed(right))
        {
            (true, false) => -1.,
            (false, true) => 1.,
            _ => 0.,
        };

        let ver: f32 = match (self.pressed(back), self.pressed(forward))
        {
            (true, false) => -1.,
            (false, true) => 1.,
            _ => 0.,
        };

        (hor, ver)
    }
    pub fn new() -> Self {
        Self {
            pressed: HashSet::new(),
            just_pressed: HashSet::new(),
            just_released: HashSet::new(),
            _cross: Vec2 { x: 0., y: 0. },
        }
    }
    pub fn pressed(&self, key: A) -> bool {
        self.pressed.contains(&key)
    }
    pub fn just_pressed(&self, key: A) -> bool {
        self.just_pressed.contains(&key)
    }
    pub fn just_released(&self, key: A) -> bool {
        self.just_released.contains(&key)
    }
    pub fn debug(&self) {
        if !(self.pressed.is_empty()
            && self.just_pressed.is_empty()
            && self.just_released.is_empty())
        {
            // Colorize them
            let pressed = format!("{:?}", self.pressed).green();
            let just_pressed = format!("{:?}", self.just_pressed).green();
            let just_released = format!("{:?}", self.just_released).green();

            println!(
                "pressed: {pressed}, just_pressed {just_pressed}, just_released {just_released}"
            );
        }
    }
    pub fn print(&self, key: A) {
        let (pressed, just_pressed, just_released) = (
            self.pressed(key.clone()),
            self.just_pressed(key.clone()),
            self.just_released(key.clone()),
        );

        if !(pressed || just_pressed || just_released)
        {
            return;
        }

        let pressed = match pressed
        {
            false => format!("{pressed}").white(),
            true => format!("{pressed}").green(),
        };

        let just_pressed = match just_pressed
        {
            false => format!("{just_pressed}").white(),
            true => format!("{just_pressed}").green(),
        };

        let just_released = match just_released
        {
            false => format!("{just_released}").white(),
            true => format!("{just_released}").green(),
        };

        let formatted = format!(
            "is: pressed {pressed}, just pressed {just_pressed}, just released {just_released}"
        );
        println!("{formatted}");
    }
}
