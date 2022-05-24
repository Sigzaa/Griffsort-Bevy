pub(crate) mod shared;
pub mod client;
pub mod server;
pub mod plugin;

pub mod prelude{
    pub use crate::shared::resources::*;
    pub use crate::shared::data_structs::*;

}

