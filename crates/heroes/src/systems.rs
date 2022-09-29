use bevy::prelude::*;

use crate::*;

pub(crate) fn sync_selected(
    q_core: Query<(Entity, &Id, &CameraLink), (With<Hero>, Without<Dead>)>,
    selected_id: Res<SelectedId>,
    mut commands: Commands,
    mut q_camera: Query<(&mut Camera, Entity)>,
) {
    for (ent, id, cam_link) in q_core.iter()
    {
        let (mut camera, c_ent) = q_camera.get_mut(cam_link.0).unwrap();

        if Some(id.0) != selected_id.0
        {
            commands.entity(c_ent).remove::<SelectedCamera>();
            commands.entity(ent).remove::<Selected>();
            camera.is_active = false;
        }
        else
        {
            commands.entity(ent).insert(Selected);
            commands.entity(c_ent).insert(SelectedCamera);
            camera.is_active = true;
        }
    }
}
