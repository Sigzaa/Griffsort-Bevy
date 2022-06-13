pub mod shared;
pub mod client;
pub mod server;
pub mod plugin;
pub use plugin::Networking;

pub mod prelude{
    pub use crate::shared::resources::*;
    pub use crate::shared::data_structs::{
        *,
        a_list::*,
        go_history::*,
    };

}

