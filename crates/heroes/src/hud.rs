use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::*, egui::*};

use crate::*;

pub(crate) fn hp_bar(
    mut egui_context: ResMut<EguiContext>,
    q_non_sel: Query<(&Hp, &MaxHp, &Team), (With<Hero>, Without<Selected>)>,
){
    for (hp, max_hp,  team) in &q_non_sel{
        
        egui::Window::new("Hp Bar")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .frame(Frame{
            rounding: Rounding::from(2.),
            shadow: egui::epaint::Shadow { extrusion: 0., ..Default::default() },
            ..Default::default()
        })
        .show(egui_context.ctx_mut(), |ui| {
            ui.add(ProgressBar::new((max_hp.0 / hp.0) as f32));
        });
    }

}