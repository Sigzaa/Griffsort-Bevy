pub mod shared;
pub mod client;
pub mod server;
pub mod plugin;
pub use plugin::Networking;
pub use bevy_snap::*;
pub use bevy::{prelude::*, reflect::TypeRegistry};
pub use crate::shared::resources::*;
pub use crate::shared::data_structs::{
        *,
        a_list::*,
        go_history::*,
};

pub mod prelude{
    pub use crate::shared::resources::*;
    pub use crate::shared::data_structs::{
        *,
        a_list::*,
        go_history::*,
    };

}

