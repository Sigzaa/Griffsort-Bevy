pub mod shared;
pub mod client;
pub mod server;
pub mod plugin;

pub use plugin::Reactive;
pub use serde::{Serialize, Deserialize};
pub use bevy::{prelude::*, reflect::TypeRegistry};
pub use shared::resources::SnapServer;
pub use shared::resources::snapshot;
pub use bevy_snap::*;

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
