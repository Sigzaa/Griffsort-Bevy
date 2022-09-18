mod plugin;
mod resources;
mod systems;

pub use plugin::UI;

pub(crate) mod resys {
    pub use super::{resources::*, systems::*};
}
