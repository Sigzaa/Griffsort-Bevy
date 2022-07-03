use bevy::prelude::*;
use super::resources::*;

pub fn hide_cursor(mut is_hided: ResMut<CursorIsHided>) {
    is_hided.0 = true;
}
pub fn unhide_cursor(mut is_hided: ResMut<CursorIsHided>) {
    is_hided.0 = false;
}
use crate::states::GameState;
pub fn alt_switch_cursor(
    keys: ResMut<Input<KeyCode>>,
    mut is_hided: ResMut<CursorIsHided>,
    mut state: ResMut<State<GameState>>
){
    if keys.just_pressed(KeyCode::LAlt) && state.current() != &GameState::AltTabbed{
        is_hided.0 = false;
        state.push(GameState::AltTabbed);
    }
    else if keys.just_pressed(KeyCode::LAlt) && state.current() == &GameState::AltTabbed{
        is_hided.0 = true;
        state.pop();
    }
}

pub fn handle_cursor(is_hided: Res<CursorIsHided>, mut windows: ResMut<Windows>){
    let window = windows.get_primary_mut().unwrap();
    if is_hided.0{
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    } else{
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}
