use bevy::reflect::TypeUuid;
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
#[derive(Default)]
pub struct ReloadRequest;