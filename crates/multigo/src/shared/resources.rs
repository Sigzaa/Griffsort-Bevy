use corgee::{GoInputs, GoRot};
use serde::{Serialize, Deserialize};
use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_snap::*;
use std::collections::HashMap;
use super::data_structs::go_history::*;

// Snapshots -->
#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct SnapShot;

impl SnapType for SnapShot {
    fn add_types(registry: &mut TypeRegistry) {
        registry.write().register::<Transform>();
    }
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum GenericMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
    ClientInputs {
        id: i32,
        tick: i32,
        inputs: [Inputs; 4],
    },
    World{
        tick: i32,
        //inputs: Vec<Inputs>, // TODO: remove it and handle with snapshots.
        snap: SnapShot,
    },
    Chat{
        id: i32,
        tick: i32,
        //msg: &'static str,
    },
    ChooseCharacter{
        code: f32
    }
}
#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ClientMessages {
    
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Inputs{
    pub ginp: GoInputs,
    pub gorot: GoRot,
}
#[derive(Default)]
pub(crate) struct SnapBuffer(pub History<SnapShot>);

#[derive(Component, Clone)]
pub(crate) struct InputsBuffer(pub History<Inputs>); // Collecting all 
// <--

#[derive(Default)]
pub struct TickCount(pub i32);

pub const PhysNet: &str = "net_stage_label"; 

pub(crate) const CONST_TICKRATE: f64 = 2.;
pub(crate) const BUFFER_CAPACITY: i32 = 200;

#[derive(Default)]
pub(crate) struct TickRate(pub bevy::prelude::Timer);


#[derive(Default)]
pub struct IsStarted(pub bool);
