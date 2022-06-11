use bevy_rapier3d::prelude::{AsyncCollider, AsyncSceneCollider};
use bevy::prelude::*;
use bevy::utils::HashMap;

pub(crate) fn load_map(
    mut commands: Commands, 
    ass: Res<AssetServer>
) {
    let handle = ass.load("../assets/models/TestMap/Test_map.gltf#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    commands.spawn_bundle(TransformBundle {
        local: Transform::from_xyz(2.0, 0.0, -5.0),
        global: GlobalTransform::identity(),
    }).with_children(|parent| {
        parent.spawn_scene(handle.clone());
    })
    .insert(AsyncSceneCollider{
        handle,
        shape: None,
        named_shapes: HashMap::new(),
    });


}
