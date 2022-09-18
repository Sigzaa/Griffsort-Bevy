use super::systems::*;
use bevy::prelude::*;
pub struct Level;
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_map)
            .add_startup_system(load_custom_models);
    }
}
