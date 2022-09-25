use bevy::{prelude::{ResMut, Bundle, Component, Commands}, ecs::{storage::Table, system::EntityCommands}};
use bevy_inspector_egui::plugin::InspectorWindows;
use bevy_rapier3d::na::Storage;
use crate::{commands_impl::toggle_visibility, characters_impl::*};

pub fn conf_commands(name: &str, mut inspector_windows: ResMut<InspectorWindows>){
    match name{
        "soul" => toggle_visibility::<soul::resources::SoulConfig>(&mut inspector_windows),
        "jacqueline" | "jac" => toggle_visibility::<jacqueline::resources::JacquelineConfig>(&mut inspector_windows),
        "yui_shang" | "shang" | "yui" => toggle_visibility::<yui_shang::resources::ShangConfig>(&mut inspector_windows),
        _ => {}
    }
}

pub fn spawn_hero(name: String, mut ec: EntityCommands){

    let name: &str = &name;

    match name {
        "soul" => ec.insert(Soul),
        "jacqueline" | "jac" => ec.insert(Jacqueline),
        "yui_shang" | "shang" | "yui" => ec.insert(YuiShang),
        _=> &mut ec
    };
    
}