use bevy_rapier3d::prelude::{AsyncSceneCollider, *};
use bevy::prelude::*;

pub(crate) fn load_map(
    mut commands: Commands, 
    ass: Res<AssetServer>,
) {
    let handle = ass.load("models/TestMap/Test_map.gltf#Scene0");
    commands.spawn_bundle(SceneBundle {
        scene: handle.clone(),
        ..Default::default()
    })
    .insert(AsyncSceneCollider {
        handle: handle,
        shape: Some(ComputedColliderShape::TriMesh),
        named_shapes: Default::default(),
    })
    .insert(RigidBody::Fixed)
    .insert(CollisionGroups::new(0b10001, 0b10000));

}
pub(crate) fn load_custom_models(   
    mut commands: Commands, 
    ass: Res<AssetServer>,
){
    // commands.spawn_bundle(SceneBundle {
    //     scene: ass.load("models/shield.gltf#Scene0"),
    //     transform: Transform{
    //         translation: Vec3::new(0.,15.,0.), 
    //         scale: Vec3::new(2.,2.,2.),
    //         ..Default::default()
    //     },
    //     ..Default::default()
    // })
    // .insert(Collider::cuboid(0.5, 2.0, 0.5))
    // .insert(RigidBody::Dynamic)
    ;
    


    commands.spawn_bundle(SceneBundle {
        scene: ass.load("models/weapon_1/weapon_1.gltf#Scene0"),
        transform: Transform{
            translation: Vec3::new(0.,0.,0.), 
            //scale: Vec3::new(1.,1.,1.),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Collider::cuboid(0.5, 2.0, 0.5))
    .insert(RigidBody::Dynamic);
}