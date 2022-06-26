
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Startup,
    MainMenu,
    InGame,
    Paused,
    Inspector
}