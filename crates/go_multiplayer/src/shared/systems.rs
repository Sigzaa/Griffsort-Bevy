use bevy::{prelude::*, reflect::TypeRegistry};
use bevy_snap::*;
use super::resources::*;

pub fn update_tick(mut tick: ResMut<TickCount>) {
    tick.0 += 1;
}
// pub fn save_snap(mut commands: Commands) {
//         // This triggers saving the world the next time commands are processed.
//         // The snapshot is then sent as an event so it can be picked up by other systems.
//         commands.save::<SnapShot>();

// }

// pub(crate) fn store_snapshot(
//     mut save_events: EventReader<SaveEvent<SnapShot>>,
//     mut save_slot: ResMut<SnapBuffer>,
//     tick: Res<TickCount>,
// ) {
//     for save_event in save_events.iter() {


//         // Save the snapshot in a resource so we can restore it later
//         //save_slot.0.push(save_event.snapshot.clone(), tick.0);

//     }
// }
// pub(crate) fn load_snap(mut commands: Commands, keys: Res<Input<KeyCode>>, save_slot: ResMut<SnapBuffer>){
//     if keys.just_pressed(KeyCode::E) {
        
//         //commands.load::<SnapShot>(save_slot.0[TICKRATE as usize].clone())
//     }
// }