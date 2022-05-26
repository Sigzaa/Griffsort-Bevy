pub mod plugin;
pub mod shared;
pub use plugin::Gocha;
mod characters;

pub mod prelude{
    pub use crate::shared::resources::*;
}
