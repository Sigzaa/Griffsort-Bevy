pub mod plugin;
pub mod shared;
pub use plugin::Gocha;
pub(crate) mod characters;

pub mod prelude{
    pub use crate::shared::resources::*;
    pub use crate::shared::*;
}
