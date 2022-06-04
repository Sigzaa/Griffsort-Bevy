pub mod plugin;
pub mod shared;
pub mod controller;

pub use plugin::CharController;
pub use crate::shared::resources::*;
pub use crate::shared::*;
pub use controller::{resources::*, plugin::*};
