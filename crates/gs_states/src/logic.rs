use super::*;
use bevy::prelude::*;

pub fn cursor_showed(state: Res<CurrentState<CursorState>>) -> bool {
    return match state.0
    {
        CursorState::Hided => true,
        CursorState::Showed => false,
    };
}

pub fn hide_cursor(mut state: ResMut<CurrentState<CursorState>>) {
    state.0 = CursorState::Hided;
}
pub fn unhide_cursor(mut state: ResMut<CurrentState<CursorState>>) {
    state.0 = CursorState::Showed;
}

pub fn handle_cursor(state: ResMut<CurrentState<CursorState>>, mut windows: ResMut<Windows>) {
    if !state.is_changed()
    {
        return;
    }

    let window = windows.get_primary_mut().unwrap();

    match state.0
    {
        CursorState::Hided =>
        {
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        }
        CursorState::Showed =>
        {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }
    }
}

pub fn alt_switch_cursor(
    keys: ResMut<Input<KeyCode>>,
    mut state: ResMut<CurrentState<CursorState>>,
) {
    if keys.just_pressed(KeyCode::LAlt)
    {
        match state.0
        {
            CursorState::Hided =>
            {
                state.0 = CursorState::Showed;
            }
            CursorState::Showed =>
            {
                state.0 = CursorState::Hided;
            }
        }
    }
}
