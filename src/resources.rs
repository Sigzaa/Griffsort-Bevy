use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Action {
    Shoot,
    Jump,
    Sprint,
    Left,
    Right,
    Back,
    Forward,
    Abil2,
    Abil1,
    Abil3,
    Ult,
    ToggleInspector,
    Command(String),
}