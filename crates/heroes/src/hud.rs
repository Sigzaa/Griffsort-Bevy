use bevy::prelude::*;
use bevy_inspector_egui::{bevy_egui::*, egui::*};
use bevy_rapier3d::parry::transformation;

use crate::*;

pub(crate) fn hp_bar(
    mut egui_context: ResMut<EguiContext>,
    q_non_sel: Query<(&Hp, &MaxHp, &Team, &Transform, Entity), (With<Hero>, Without<Selected>)>,
    camera: Query<(&Camera, &GlobalTransform), With<SelectedCamera>>,
    config: Res<HeroesConfig>,
) {
    let (cam, cam_transform) = match camera.get_single()
    {
        Ok(cam) => cam,
        _ => return,
    };

    for (hp, max_hp, team, transform, entity) in &q_non_sel
    {
        let pos = match cam.world_to_viewport(cam_transform, transform.translation)
        {
            Some(pos) => pos,
            _ => continue,
        };

        if max_hp.0 <= 0
        {
            continue;
        }

        let size = config.hp_bar_size;

        egui::Window::new(format!("HpBar {entity:?}"))
            .title_bar(false)
            .collapsible(false)
            .resizable(false)
            .fixed_pos([pos[0] - size[0] / 2., -pos[1] + 1300.])
            .fixed_size(size)
            .frame(Frame {
                rounding: Rounding::from(2.),
                shadow: egui::epaint::Shadow {
                    extrusion: 0.,
                    ..Default::default()
                },
                ..Default::default()
            })
            .show(egui_context.ctx_mut(), |ui| {
                if hp.0 > 0
                {
                    ui.add(ProgressBar::new(hp.0 as f32 / max_hp.0 as f32));
                }
            });
    }
}
