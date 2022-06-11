use bevy::prelude::*;
use super::systems::*;

pub struct Level;
impl Plugin for Level {
    fn build(&self, app: &mut App) {
        app
        .add_system(load_map)    
        ;
    }
}
