mod commands;
mod plugin;
//mod resources;
mod systems;
mod widgets;
mod resources;

pub use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin, *};
pub use plugin::{Inspector};
pub use resources::*;
