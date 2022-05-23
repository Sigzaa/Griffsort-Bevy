pub mod goinputs;
mod shared;
pub mod plugin;

pub mod prelude {
    pub use crate::goinputs::{resources::GoInputs, *};
    pub use crate::shared::resources::*;
}


