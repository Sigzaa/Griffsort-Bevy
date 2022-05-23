use bevy::{  prelude::*, };
mod spawn;
mod abilities;
pub struct Soul;
impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app
            .add_system(spawn::spawn)
            .add_system(abilities::large_shield)
            .add_system(abilities::shield_deployment)
            //.add_system(SoulS.movement)
            //.add_plugin(characters::soul::Soul)
            ;
    }
}
