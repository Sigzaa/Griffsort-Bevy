pub mod goinputs;
pub mod plugin;
mod shared;
pub mod additional;

pub use plugin::Corgee;
pub use crate::goinputs::{resources::GoInputs, resources::*, *};
pub use crate::shared::{resources::*, character::*, level::*,  *};
pub use additional::*;