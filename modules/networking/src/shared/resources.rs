use core::prelude::GoInputs;
use gocha::prelude::GoRot;
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, reflect::TypeRegistry};
pub use priority_queue::PriorityQueue;
use bevy_snap::*;
use super::data_structs::go_buf::*;

// Snapshots -->
#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug)]
pub struct SnapShot;

impl SnapType for SnapShot {
    fn add_types(registry: &mut TypeRegistry) {
        registry.write().register::<Transform>();
    }
}
// <--


// Serde pack sending/receiving via UDP -->
#[derive(Eq, PartialEq, Hash)]
pub enum BufContent{
    SnapShot,
    Inputs,
}
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Inputs{
    pub ginp: GoInputs,
    pub grot: GoRot,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct MsgPack {
    pub id: i32,
    pub tick: i32,
    pub inputs: Option<Vec<Inputs>>,
    pub snap: Option<SnapShot>,
}
// <--
#[derive(Default)]
pub struct IsStarted(pub bool);

struct FrameCount(u32);

#[derive(Default)]
pub struct TickCount(pub i32);

pub const NetStage: &str = "net_stage_label"; 

pub(crate) const TICKRATE: i32 = 66;

#[derive(Default)]
pub struct IternalShots(GoBuf); // Collecting all client-made SnapShots in Buffer

#[derive(Default)]
struct ServerShot(GoBuf); // Collecting all 

#[derive(Component)]
pub struct InputsHistory(GoBuf);

