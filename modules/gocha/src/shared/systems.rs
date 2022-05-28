use bevy::prelude::*;
use super::resources::*;

fn confirm_go_rotation(){
    todo!();
}
fn gravity(){
    todo!();
}
fn register_hit(){
    todo!();
}
fn spawn_projectile(){
    todo!();
}
fn velocity_to_position(){
    todo!();
}
fn sync_current_mode(){
    todo!();
}
fn sync_current_character(){
    todo!();
}
fn sync_id(){
    todo!();
}
pub(crate) fn spawn_char(
    mut spawn_request: EventReader<SpawnCharacter>,
){
    for spawn_info in spawn_request.iter() {
        let team = spawn_info.1;
        //spawn_info.2
    }
}
fn reload_config(){
    todo!();
}
