use bevy_rapier3d::prelude::{AsyncCollider, AsyncSceneCollider, *};
use bevy::prelude::*;
use super::resources::*;
use bevy::utils::HashMap;
use go_core::Map;

pub(crate) fn load_map(
    mut commands: Commands, 
    ass: Res<AssetServer>,
    map: Res<State<Map>>,
) {
    let handle = ass.load("../assets/models/TestMap/Test_map.gltf#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    let mut scene_commands = commands.spawn_bundle(TransformBundle::default());

    scene_commands
    .insert(AsyncSceneCollider {
        handle: handle.clone(),
        shape: Some(ComputedColliderShape::TriMesh),
        named_shapes: Default::default(),
    })
    .insert(RigidBody::Fixed)
    // .insert(CollisionGroups {
    //     memberships: CollisionMask::WORLD.bits(),
    //     filters: CollisionMask::all().bits(),
    // })
    // .insert(InGameOnly)
    .with_children(|parent| {
        parent.spawn_scene(handle);
    });


}
