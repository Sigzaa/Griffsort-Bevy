use bevy::prelude::*;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Default)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"] 
pub struct ClientConfig{
    pub mouse_sensitivity: f32,
    pub fullscreen: bool,
    pub resolution: (u16, u16),
    pub vsync: bool,
    pub fov: f32,
   // pub fps_limit: FpsLimit
}
#[derive(serde::Deserialize, bevy::reflect::TypeUuid, Clone)]
#[uuid = "fcae77b0-26df-11ed-a261-0242ac120002"]
pub struct InputMap {
    pub jump: KeyCode, // Space
    pub shoot: MouseButton, // Left Mouse Button
    pub forward: KeyCode,
    pub left: KeyCode,
    pub right: KeyCode,
    pub back: KeyCode,
    pub a_1: MouseButton, // Right Mouse Button
    pub a_2: KeyCode, // Q
    pub a_3: KeyCode, // F
    pub ult: KeyCode, // E
    pub esc: KeyCode, // Shift
    pub crouch: KeyCode, // Ctrl
}
impl InputMap{
    pub fn default() -> Self {
        Self{
        jump: KeyCode::Space, // Space
        shoot: MouseButton::Left, // Left Mouse Button
        forward: KeyCode::W,
        left: KeyCode::A,
        right: KeyCode::D,
        back: KeyCode::S,
        a_1: MouseButton::Right, // Right Mouse Button
        a_2: KeyCode::Q, // Q
        a_3: KeyCode::F, // F
        ult: KeyCode::E, // E
        esc: KeyCode::LShift, // Shift
        crouch: KeyCode::LControl, // Ctrl
        }
    }

}
pub enum Key {
    Board(KeyCode),
    Mouse(MouseButton)
}

#[derive(Default)]
pub struct ReloadRequest;