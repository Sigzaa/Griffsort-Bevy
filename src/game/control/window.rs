use bevy::prelude::*;
use crate::game::components::*;

pub(crate) fn window_control(
    input: Res<Input<KeyCode>>,

) {
    if input.pressed(KeyCode::Delete) {
        std::process::exit(0);
    }
}
pub fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
    mut grabbed_flag: ResMut<GrabbedCursor>,
    
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
        grabbed_flag.0 = true;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
        grabbed_flag.0 = false;
    }
}