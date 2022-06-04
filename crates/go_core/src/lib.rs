pub mod goinputs;
pub mod plugin;
mod shared;

pub use plugin::Core;
pub use crate::goinputs::{resources::GoInputs, resources::*, *};
pub use crate::shared::resources::*;
