mod plugin;
mod resources;
mod widgets;
mod commands;
mod systems;

pub use plugin::Inspector;
pub use bevy_console::{ConsoleConfiguration, ConsolePlugin, AddConsoleCommand, ConsoleCommand, *};
