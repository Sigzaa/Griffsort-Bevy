
use bevy::prelude::*;
use crate::game::components::{*, filters::*, player_data::* };


pub fn control_bind( // Binds camera and <Selected>  according to BindedId resource.
    binded_id: Res<BindedId>,
    cam: Query<Entity, With<ThreeDCam>>,
    sel: Query<(Entity, &Id), With<Selected>>,
    q_core: Query<(&Id, Entity), (With<Core>, Without<Killed>)>,
    mut commands: Commands,
){
    let cam_ent = cam.single();

    for (sel_ent, id )in sel.iter(){
        if id.0 == binded_id.0{
            return;
        }
        println!("removing cam");
        commands.entity(sel_ent).remove::<Selected>();
        commands.entity(sel_ent).remove_children(&[cam_ent]);
    }
    for (id, ent)in q_core.iter(){
        if id.0 == binded_id.0{
            println!("pushing cam");
            commands.entity(ent).push_children(&[cam_ent]);
            commands.entity(ent).insert(Selected);
        }
    }
}

/*
use crate::game::components::{ Filters::*, *, player_states::*};
pub fn switch_selection(
    mut q_camera: Query<Entity, With<ThreeDCam>>,

    mut q_core: Query<(&Id, Entity, Option<&Selected>), (With<Core>, Without<Killed>)>,
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
) {

    let camera = q_camera.single_mut();
    let mut new_id = 0;
    if input.just_pressed(KeyCode::Tab) {
    for (id, entity, selected) in q_core.iter_mut() {

        if let Some(selected) = selected{
            
        }

            //", id.value);
            
            if MAXPLAYERCOUNT > id {
                new_id = id.value + 1;
            } else {
                new_id = 1;
            }
        
    }
        if let Some(core_id) = core_id {
            if core_id.value == new_id {
                //commands.entity(core_id).remove::<SelectedHead>();
                commands.entity(sel_core).remove::<Selected>();
                //commands.entity(head).insert(SelectedHead);
                commands.entity(sel_core).remove_children(&[camera]);
                commands.entity(core).push_children(&[camera]);
                commands.entity(core).insert(Selected);
            }
        }
    }
    for (core, dev_null, core_id) in q_core.iter_mut() {
        if let Some(core_id) = core_id {
            if core_id.value == new_id {
                //commands.entity(core).insert(Selected);
            }
        }
    }
}
*/