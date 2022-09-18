mod default;
mod soul;
mod tamara;
mod zero;

use bevy::prelude::*;
use heroes::*;

#[derive(Copy, Clone, Component)]
pub struct Soul;

#[derive(Copy, Clone, Component)]
struct Tamara;

#[derive(Copy, Clone, Component)]
struct Zero;

pub struct CharactersImpl;
impl Plugin for CharactersImpl {
    fn build(&self, app: &mut App) {
        app.add_plugin(Controller::<Zero>::new(Zero))
            //.add_plugin(Controller::<Tamara>::new(Tamara))
            .add_plugin(Controller::<Soul>::new(Soul));
    }
}

#[derive(Component)]
pub struct PointingOn(pub Vec<Data>);

#[derive(Component)]
pub struct Crosshair;

pub struct Data {
    pub toi: Toi,
    pub target: Entity,
}
