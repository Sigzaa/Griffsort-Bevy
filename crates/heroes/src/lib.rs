pub mod plugin;
pub mod controller;

pub use plugin::CharController;
pub use controller::{resources::*, plugin::*,prebuild::*};
pub use bevy_atmosphere::prelude::*;
