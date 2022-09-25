use bevy::prelude::*;

use crate::*;

pub(crate) fn sync_selected(
    q_core: Query<(Entity, &Id, &CameraLink), (With<Hero>, Without<Dead>)>,
    selected_id: Res<SelectedId>,
    mut commands: Commands,
    mut q_camera: Query<&mut Camera>,
) {
    for (ent, id, cam_link) in q_core.iter() {
        let mut camera = q_camera.get_mut(cam_link.0).unwrap();

        if Some(id.0) != selected_id.0 {
            commands.entity(ent).remove::<Selected>();
            camera.is_active = false;
        } else {
            commands.entity(ent).insert(Selected);
            camera.is_active = true;
        }
    }
}

pub(crate) fn clear_despawned_heroes(
    
){

}