use crate::game::components::player_data::*;
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, reflect::TypeRegistry};
use priority_queue::PriorityQueue;
use bevy_snap::*;

#[derive(Default)]
pub struct SnapShot;

#[derive(Default)]
pub struct SaveSlot(pub Vec<WorldSnapshot<SnapShot>>);



#[derive(Component)]
pub struct Buffer(pub PriorityQueue<MsgPack, i32>);


use std::hash::{Hash, Hasher};

impl Hash for MsgPack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tick.hash(state);
        //self.phone.hash(state);
    }
}

impl PartialEq for MsgPack {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
    }
}
impl Eq for MsgPack {}

#[derive(Component, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct MsgPack {
    pub id: i32,
    pub tick: i32,
    pub ctrl: Control,
    pub rotation: Quat,
    pub head_rotation: Quat,
}