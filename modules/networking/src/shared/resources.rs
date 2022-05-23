use core::prelude::GoInputs;
use gocha::prelude::GoRot;
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, reflect::TypeRegistry};
pub use priority_queue::PriorityQueue;
use bevy_snap::*;

// Snapshots -->
#[derive(Default)]
pub struct SnapShot;

impl SnapType for SnapShot {
    fn add_types(registry: &mut TypeRegistry) {
        registry.write().register::<Transform>();
    }
}
#[derive(Default)]
pub struct SnapStorage(pub Vec<WorldSnapshot<SnapShot>>);

#[derive(Default)]
pub struct ReceivedSnapStorage(pub Vec<WorldSnapshot<SnapShot>>);
// <--

// Buffer -->
use std::hash::{Hash, Hasher};

#[derive(Component)]
pub struct Buffer(pub PriorityQueue<MsgPack, i32>);

impl Hash for MsgPack {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.tick.hash(state);
    }
}
impl PartialEq for MsgPack {
    fn eq(&self, other: &Self) -> bool {
        self.tick == other.tick
    }
}
impl Eq for MsgPack {}
// <--

// Serde pack sending/receiving via UDP -->
#[derive(Component, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct MsgPack {
    pub id: i32,
    pub tick: i32,
    pub ginp: GoInputs,
    pub grot: GoRot,
}
pub enum Msg{
    Input, // MsgPack
    Output, // Position, states
    Chat
}
// <--
#[derive(Default)]
pub struct IsStarted(pub bool);

struct FrameCount(u32);

pub const NetStage: &str = "net"; 

pub const TICKRATE: i32 = 66;

#[derive(Default)]
pub struct TickCount(pub i32);