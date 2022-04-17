use crate::game::components::player_data::Control;
use bevy::prelude::*;
use priority_queue::PriorityQueue;

#[derive(Component)]
pub struct Buffer(pub PriorityQueue<MsgPack, i32>);

#[derive(Component, Clone, Copy, Debug)]
pub struct MsgPack {
    pub ctrl: Control,
    pub id: i32,
    pub rotation: Vec4,
    pub tick: i32,
}

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

#[derive(Component, Clone, Copy, Debug)]

pub struct msg_structure {
    pub ctrl: Control,
    pub rotation: Quat,
    pub head_rotation: Quat,
    pub id: i32,
    pub tick: i32,
}
impl msg_structure {
    pub fn pack(&self) -> String {
        let (_x, y, _z, w) = quat_unpack(self.rotation); // Looks ugly and difficoultly.
        let (x1, _y1, _z1, w1) = quat_unpack(self.head_rotation); // Also this.

        let ctrl_msg = ctrl_to_string(self.ctrl);

        format!(
            "{} {} {} {} {} {} {}",
            self.id, self.tick, y, w, x1, w1, ctrl_msg,
        )
    }
}
fn ctrl_to_string(ctrl: Control) -> String {
    // Looks schlecht (bad) 0_o.
    // mey be better is iterate in the struct.
    format!(
        "{} {} {} {} {} {} {} {} {} {}",
        i8::from(ctrl.forward),
        i8::from(ctrl.left),
        i8::from(ctrl.right),
        i8::from(ctrl.back),
        i8::from(ctrl.q),
        i8::from(ctrl.lmb),
        i8::from(ctrl.rmb),
        i8::from(ctrl.jump),
        i8::from(ctrl.shift),
        i8::from(ctrl.e),
    )
    //String::from("hey")
}

fn string_to_ctrl(v: Vec<f32>) -> Control {
    Control {
        forward: to_bool(v[6]),
        left: to_bool(v[7]),
        right: to_bool(v[8]),
        back: to_bool(v[9]),
        q: to_bool(v[10]),
        lmb: to_bool(v[11]),
        rmb: to_bool(v[12]),
        jump: to_bool(v[13]),
        shift: to_bool(v[14]),
        e: to_bool(v[15]),
        f: false,
        ..Default::default()
    }
}
pub fn pack_msg(msg: msg_structure) -> bool {
    true
}
pub fn unpack_msg(msg: &[u8]) -> msg_structure{

    let v: Vec<f32> = split(msg);
    
    msg_structure{
        ctrl: string_to_ctrl(v.clone()),
        id: v[0] as i32,
        tick: v[1] as i32,
        head_rotation: quat_pack(0.,0.,0.,0.),
        rotation: quat_pack(0.,0.,0.,0.),

    }
}
pub fn msg_to_MsgPack(msg: &[u8]) -> MsgPack{
    
    let v = split(msg);

    MsgPack{
        ctrl: string_to_ctrl(v.clone()),
        id: v[0] as i32,
        tick: v[1] as i32,
        rotation: Vec4::new(v[4], v[2], v[5], v[3])

    }
}

pub fn split(msg: &[u8]) -> Vec<f32>{
    String::from_utf8_lossy(msg)
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect()
}
pub fn quat_unpack(q: Quat) -> (f32, f32, f32, f32) {
    let v = Vec4::from(q);
    (v[0], v[1], v[2], v[3])
}
pub fn quat_pack(x: f32, y: f32, z: f32, w: f32) -> Quat {
    Quat::from_xyzw(x, y, z, w)
}
fn to_bool(num: f32) -> bool {
    if num == 0. {
        false
    } else {
        true
    }
}
