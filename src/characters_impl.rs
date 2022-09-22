mod default;
pub mod soul;
pub mod tamara;
pub mod zero;
pub mod yui_shang;
pub mod jacqueline;

use bevy::prelude::*;
use heroes::*;

#[derive(Copy, Clone, Component)]
pub struct Soul;

#[derive(Copy, Clone, Component)]
struct Tamara;

#[derive(Copy, Clone, Component)]
struct Zero;

#[derive(Copy, Clone, Component)]
pub struct YuiShang;

#[derive(Copy, Clone, Component)]
pub struct Jacqueline;

pub struct Characters;
impl Plugin for Characters {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Controller::<Zero>::new(Zero))
            .add_plugin(Controller::<YuiShang>::new(YuiShang))
            .add_plugin(Controller::<Jacqueline>::new(Jacqueline))
            .add_plugin(Controller::<Soul>::new(Soul));
    }
}

