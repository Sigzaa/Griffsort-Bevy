use core::prelude::{GoInputs, GoRot};
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, reflect::TypeRegistry};
pub use priority_queue::PriorityQueue;
use bevy_snap::*;
use super::data_structs::go_history::*;

// Snapshots -->
#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct SnapShot;

impl SnapType for SnapShot {
    fn add_types(registry: &mut TypeRegistry) {
        registry.write().register::<Transform>();
    }
}
// <--

// Serde pack sending/receiving via UDP -->

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Inputs{
    pub ginp: GoInputs,
    pub grot: GoRot,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct FromClient {
    pub id: i32,
    pub tick: i32,
    pub inputs: Inputs,
}
#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct FromServer {
    pub tick: i32,
    pub inputs: Vec<Inputs>,
    pub snap: SnapShot,
}
#[derive(Default)]
pub(crate) struct SnapBuffer(pub GoHistory<SnapShot>);

#[derive(Component, Clone)]
pub(crate) struct InputsBuffer(pub GoHistory<Inputs>); // Collecting all 
// <--
#[derive(Default)]
pub struct IsStarted(pub bool);

struct FrameCount(u32);

#[derive(Default)]
pub struct TickCount(pub i32);

pub const NetStage: &str = "net_stage_label"; 

pub(crate) const TICKRATE: i32 = 66;
pub(crate) const BUFFER_CAPACITY: i32 = 200;


