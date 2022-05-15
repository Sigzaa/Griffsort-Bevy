use bevy::{prelude::*, reflect::TypeRegistry};
use crate::game::networking::shared::resources::*;
use bevy_snap::*;


impl SnapType for SnapShot {
    fn add_types(registry: &mut TypeRegistry) {
        // Register the types you want to be saved and loaded
        registry.write().register::<Transform>();
    }
}

// Resources also need to implement the Reflect and Component traits trait
#[derive(Component, Reflect, Default)]
// Resources also (at least at the moment) need to be marked as Components, as well as Resources
#[reflect(Component, Resource)]
pub struct Steps(pub f32);

// Actual save data is contained in the WorldSnapshot type,
// which is generic over your type of snapshot.
