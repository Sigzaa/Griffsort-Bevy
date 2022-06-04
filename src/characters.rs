mod zero;
mod soul;

use go_character::*;
use bevy::prelude::*;

#[derive(Copy, Clone, Component)]
struct Soul;

#[derive(Copy, Clone, Component)]
struct Zero;

pub struct CharactersImpl;
impl Plugin for CharactersImpl {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Controller::<Zero>::new(Zero))
            .add_plugin(Controller::<Soul>::new(Soul));
    }
}
