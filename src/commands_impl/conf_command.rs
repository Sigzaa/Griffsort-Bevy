use bevy::prelude::{Commands, ResMut};
use bevy_console::*;
use bevy_inspector_egui::{plugin::InspectorWindows, Inspectable};
use crate::characters_impl::*;

#[derive(ConsoleCommand)]
#[console_command(name = "conf")]
pub struct ConfCommand {
    /// Some message
    inspectable_name: String,
}

pub fn conf_command(mut log: ConsoleCommand<ConfCommand>, mut inspector_windows: ResMut<InspectorWindows>,) {
    if let Some(ConfCommand { inspectable_name }) = log.take() {
        match inspectable_name.as_str(){
            "soul" => change_visibility::<soul::resources::SoulConfig>(&mut inspector_windows),
            "jacqueline" | "jac" => change_visibility::<jacqueline::resources::JacquelineConfig>(&mut inspector_windows),
            "yui_shang" | "shang" | "yui" => change_visibility::<yui_shang::resources::ShangConfig>(&mut inspector_windows),
            _ => {}
        }
        // handle command
    }
}

fn change_visibility<C: Inspectable + 'static>(inspector_windows: &mut InspectorWindows){
    let mut inspector_window_data = inspector_windows.window_data_mut::<C>();
    inspector_window_data.visible = !inspector_window_data.visible;
}