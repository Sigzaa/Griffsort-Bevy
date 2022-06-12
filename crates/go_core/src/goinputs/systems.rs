use super::resources::MyGamepad;
use crate::*;
use bevy::{
    input::mouse::MouseMotion,
    prelude::{KeyCode, *},
};

pub fn collect_inputs(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut q_selected: Query<(&mut GoInputs, &mut GoRot), With<Selected>>,
    sens: Res<Sensitivity>,
    mut motion_evr: EventReader<MouseMotion>,
    time: Res<Time>
) {
    for (mut ginp, mut gorot) in q_selected.iter_mut() {
        for ev in motion_evr.iter() {
            gorot.y *= Quat::from_rotation_y(-ev.delta.x * sens.0 * time.delta_seconds());
            gorot.x *= Quat::from_rotation_x(-ev.delta.y * sens.0 * time.delta_seconds());
            //gorot.z = Quat::from_rotation_z(-ev.delta.x * SENSITIVITY); TODO!
        }

        *ginp = GoInputs::default();

        let w = input.pressed(KeyCode::W);
        let s = input.pressed(KeyCode::S);

        if !(s && w) {
            if s {
                ginp.movement[1] = -1.;
            }
            if w {
                ginp.movement[1] = 1.;
            }
        }

        let a = input.pressed(KeyCode::A);
        let d = input.pressed(KeyCode::D);

        if !(a && d) {
            if a {
                ginp.movement[0] = -1.;
            }
            if d {
                ginp.movement[0] = 1.;
            }
        }

        if buttons.pressed(MouseButton::Left) {
            ginp.fire = 1;
        }
        if input.pressed(KeyCode::Q) {
            ginp.a_1 = 1;
        }
        if input.pressed(KeyCode::E) {
            ginp.a_2 = 1;
        }
        if input.pressed(KeyCode::LShift) {
            ginp.sprint = 1;
        }
        if input.pressed(KeyCode::Space) {
            ginp.jump = 1;
        }
    }
}
pub fn camera_motion(
    mut motion_evr: EventReader<MouseMotion>,
    mut q_core: Query<&mut GoRot, With<Selected>>,
    sens: Res<Sensitivity>,
    time: Res<Time>,
) {
}
