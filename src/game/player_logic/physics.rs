use crate::game::components::{filters::*, player_data::*};
use bevy::prelude::*;

pub fn head_movement_system(
    q_parent: Query<
        (&mut Transform, &Control, &HeadRotation, &Children),
        (With<Core>, Without<CustomHeadMovement>),
    >,
    mut q_child: Query<&mut Transform, Without<Core>>,
) {
    //if &args[1] != "server" { return; }
    for (_transform, _ctrl, head_rotation, children) in q_parent.iter() {
        for &child in children.iter() {
            let mut transform = q_child.get_mut(child).unwrap();

            transform.rotation = head_rotation.0;
        }
    }
}
