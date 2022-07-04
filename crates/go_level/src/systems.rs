use bevy_rapier3d::prelude::{AsyncCollider, AsyncSceneCollider, *};
use bevy::prelude::*;
use super::resources::*;
use bevy::utils::HashMap;
use corgee::Map;

pub(crate) fn load_map(
    mut commands: Commands, 
    ass: Res<AssetServer>,
    map: Res<State<Map>>,
) {
    // let handle = ass.load("../assets/models/TestMap/Test_map.gltf#Scene0");

    // // to be able to position our 3d model:
    // // spawn a parent entity with a TransformBundle
    // // and spawn our gltf as a scene under it
    // let mut scene_commands = commands.spawn_bundle(TransformBundle::default());

    // scene_commands
    // .insert(AsyncSceneCollider {
    //     handle: handle.clone(),
    //     shape: Some(ComputedColliderShape::TriMesh),
    //     named_shapes: Default::default(),
    // })
    // .insert(RigidBody::Fixed)
    // // .insert(CollisionGroups {
    // //     memberships: CollisionMask::WORLD.bits(),
    // //     filters: CollisionMask::all().bits(),
    // // })
    // // .insert(InGameOnly)
    // .with_children(|parent| {
    //     parent.spawn_scene(handle);
    // });

    let handle = ass.load("../assets/models/weapon1_broken/weapon1_broken.gltf#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    let mut scene_commands = commands.spawn_bundle(TransformBundle{
        global: GlobalTransform::from_xyz(0., 10., 0.),
        local: Transform::from_xyz(-1., 10., 0.),
    });
    //let mut scene_commands = commands.spawn();

    scene_commands
    //.insert(Transform::from_xyz(0., 10., 0.))
    // .insert(AsyncSceneCollider {
    //     handle: handle.clone(),
    //     shape: Some(ComputedColliderShape::TriMesh),
    //     named_shapes: Default::default(),
    // })
    .insert(Collider::cuboid(0.5, 2.0, 0.5))
    .insert(RigidBody::Dynamic)

    // .insert(CollisionGroups {
    //     memberships: CollisionMask::WORLD.bits(),
    //     filters: CollisionMask::all().bits(),
    // })
    // .insert(InGameOnly)
    .with_children(|parent| {
        parent.spawn_scene(handle);
    });





}
