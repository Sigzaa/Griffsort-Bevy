mod logic;
pub use logic::*;

use bevy::prelude::*;
pub use iyes_loopless::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::MainMenu)
            .add_loopless_state(CursorState::Showed)
            .add_enter_system(GameState::InGame, hide_cursor)
            .add_system(handle_cursor)
            .add_system(alt_switch_cursor);
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    ConnectingToServer,
    LoadingAssets,
    MainMenu,
    LoadingMatchAssets,
    WaitingPlayers,
    HeroPicking,
    Preparing,
    InGame,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum CursorState {
    Hided,
    Showed,
}
