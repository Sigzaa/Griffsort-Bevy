use bevy::{prelude::*};
use crate::shared::resources::*;
use super::soul::Soul;
use super::systems::*;

pub struct Characters;
impl Plugin for Characters {
    fn build(&self, app: &mut App) {
        app 
            .add_event::<SpawnCharacter>()
            .add_system(master)
            .add_plugin(Soul)
            ;
    }
}
