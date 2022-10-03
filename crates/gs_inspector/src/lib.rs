mod commands;
mod plugin;
//mod resources;
mod systems;
mod widgets;

pub use bevy_console::{AddConsoleCommand, ConsoleCommand, ConsoleConfiguration, ConsolePlugin, *};
pub use plugin::Inspector;
