use bevy::{ prelude::*};
use crate::shared::systems::*;
use crate::shared::resources::*;

pub struct Gocha;
impl Plugin for Gocha {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(SelectedId(None))
            .insert_resource(CharList(Vec::new()))

        ;
    }
}
