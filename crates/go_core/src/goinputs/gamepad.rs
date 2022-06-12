use super::resources::*;
use crate::*;
/// Simple resource to store the ID of the connected gamepad.
/// We need to know which gamepad to use for player input.
///
use bevy::prelude::*;



pub fn gamepad_connections(
    mut commands: Commands,
    my_gamepad: Option<Res<MyGamepad>>,
    mut gamepad_evr: EventReader<GamepadEvent>,
) {
    for GamepadEvent(id, kind) in gamepad_evr.iter() {
        match kind {
            GamepadEventType::Connected => {
                println!("New gamepad connected with ID: {:?}", id);

                // if we don't have any gamepad yet, use this one
                if my_gamepad.is_none() {
                    commands.insert_resource(MyGamepad(*id));
                }
            }
            GamepadEventType::Disconnected => {
                println!("Lost gamepad connection with ID: {:?}", id);

                // if it's the one we previously associated with the player,
                // disassociate it:
                if let Some(MyGamepad(old_id)) = my_gamepad.as_deref() {
                    if old_id == id {
                        commands.remove_resource::<MyGamepad>();
                    }
                }
            }
            // other events are irrelevant
            _ => {}
        }
    }
}
pub fn gamepad_input(
    axes: Res<Axis<GamepadAxis>>,
    buttons: Res<Input<GamepadButton>>,
    my_gamepad: Option<Res<MyGamepad>>,
    mut q_sel: Query<(&mut GoInputs, &mut GoRot), With<Selected>>,
    sens: Res<Sensitivity>,
    time: Res<Time>
) {
    let gamepad = my_gamepad.unwrap().0;

    for (mut ginp, mut gorot) in q_sel.iter_mut() {
        *ginp = GoInputs::default();
        //*ginp = GoInputs::default();
        let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
        let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
            // combine X and Y into one vector
            
            let left_stick_pos = Vec2::new(x, y);
            ginp.movement = left_stick_pos;
        }
        let axis_rx = GamepadAxis(gamepad, GamepadAxisType::RightStickX);
        let axis_ry = GamepadAxis(gamepad, GamepadAxisType::RightStickY);

        if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
            gorot.y *= Quat::from_rotation_y(-x * sens.0 * 15. * time.delta_seconds());
            gorot.x *= Quat::from_rotation_x( y * sens.0 * 15. * time.delta_seconds());
        }


        

        // In a real game, the buttons would be configurable, but here we hardcode them
        let jump_button = GamepadButton(gamepad, GamepadButtonType::South);
        let heal_button = GamepadButton(gamepad, GamepadButtonType::East);

        if buttons.pressed(jump_button) {
            ginp.jump = 1;
            // button just pressed: make the player jump
        }

        if buttons.pressed(heal_button) {
            // button being held down: heal the player
        }
    }
    // The joysticks are represented using a separate axis for X and Y
}

use bevy::ecs::schedule::ShouldRun;

pub fn run_if_gamepad_connected(
    my_gamepad: Option<Res<MyGamepad>>,
) -> ShouldRun{
    if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        return ShouldRun::Yes;
    } else {
        // no gamepad is connected
        return ShouldRun::No;
    };
}
pub fn run_if_gamepad_disconnected(
    my_gamepad: Option<Res<MyGamepad>>,
) -> ShouldRun{
    if let Some(gp) = my_gamepad {
        // a gamepad is connected, we have the id
        return ShouldRun::No;
    } else {
        // no gamepad is connected
        return ShouldRun::Yes;
    };
}
