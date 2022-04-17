use bevy::prelude::*;
use crate::game::components::player_data::*;
use priority_queue::PriorityQueue;


#[derive(Default)]
pub struct ServerAddr(pub Vec<std::net::SocketAddr>); // Replace Vec by something better.

#[derive(Default)]
pub struct MyId(pub i32);

#[derive(Default)]
pub struct TickCounter(pub i32);

#[derive(Default )]
pub struct InputHistory(pub Vec<HisPack>);

#[derive(Debug)]
pub struct HisPack {
    pub input: Control,
    pub tick: i32,
    pub delta_seconds: f32,
    pub rotation: Quat,
    pub translation: Vec3,
    pub st_tr: Vec3,
}
#[derive(Component, Clone, Copy, Debug)]
pub struct MsgPack {
    pub ctrl: Control,
    pub id: i32,
    pub rotation: Vec4,
    pub tick: i32,
    pub entity: Entity,
}



#[derive(Component)]
pub struct InpBuffer(pub PriorityQueue<InpBuf, i32>);

#[derive(Component, Clone, Copy, Debug)]
pub struct InpBuf {
    pub pos: Vec3,
    pub tick: i32,
}

use std::hash::{Hash, Hasher};

impl Hash for InpBuf {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tick.hash(state);
        //self.phone.hash(state);
    }
}

impl PartialEq for InpBuf {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
    }
}
impl Eq for InpBuf {}