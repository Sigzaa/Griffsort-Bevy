use crate::{characters_impl::*, commands_impl::toggle_visibility};
use bevy::{
    ecs::{system::EntityCommands},
    prelude::{ResMut},
};
use bevy_inspector_egui::plugin::InspectorWindows;

// Hardcoded String-to-action mapping

// Toggle visibility of characters Egui configs
pub fn conf_commands(name: &str, mut inspector_windows: ResMut<InspectorWindows>) {
    match name
    {
        "soul" => toggle_visibility::<soul::resources::SoulConfig>(&mut inspector_windows),
        "jacqueline" | "jac" => toggle_visibility::<jacqueline::resources::JacquelineConfig>(&mut inspector_windows),
        "yui_shang" | "shang" | "yui" => toggle_visibility::<yui_shang::resources::ShangConfig>(&mut inspector_windows),
        "heroes" => toggle_visibility::<heroes::HeroesConfig>(&mut inspector_windows),
        _ =>
        {}
    }
}

// Extend entity
// Uses for spawning heros with string

pub fn spawn_hero(name: String, mut ec: EntityCommands) {
    let name: &str = &name;

    match name
    {
        "soul" => ec.insert(Soul),
        "jacqueline" | "jac" => ec.insert(Jacqueline),
        "yui_shang" | "shang" | "yui" => ec.insert(YuiShang),
        _ => &mut ec,
    };
}
