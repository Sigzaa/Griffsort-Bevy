use crate::heroes_mapping::conf_commands;
use bevy::prelude::ResMut;
use bevy_console::*;
use bevy_inspector_egui::{plugin::InspectorWindows, Inspectable};

#[derive(ConsoleCommand)]
#[console_command(name = "conf")]
pub struct ConfCommand {
    /// Some message
    inspectable_name: String,
}

pub fn conf_command(
    mut log: ConsoleCommand<ConfCommand>,
    inspector_windows: ResMut<InspectorWindows>,
) {
    if let Some(ConfCommand { inspectable_name }) = log.take()
    {
        conf_commands(inspectable_name.as_str(), inspector_windows);
        // handle command
    }
}

pub fn toggle_visibility<C: Inspectable + 'static>(inspector_windows: &mut InspectorWindows) {
    let mut inspector_window_data = inspector_windows.window_data_mut::<C>();
    inspector_window_data.visible = !inspector_window_data.visible;
}
