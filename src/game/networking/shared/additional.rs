use crate::game::components::player_data::Control;
use bevy::prelude::*;




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
