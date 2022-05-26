pub mod goinputs;
mod shared;
pub mod plugin;
pub use plugin::Core;

pub mod prelude {
    pub use crate::goinputs::{resources::*, *, resources::GoInputs};
    pub use crate::shared::resources::*;
}


