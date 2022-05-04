use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};

use crate::game::components::*;
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct MapBody;

#[derive(Component)]
pub struct MapCollider;

#[derive(Component, Debug)]
pub struct MapModel;

#[derive(Default)]
pub struct IsLoaded(pub bool);

#[derive(Default)]
pub struct MapHandle(pub bevy::prelude::Handle<bevy::prelude::Scene>);

use bevy::gltf::Gltf;

/// Helper resource for tracking our asset
pub struct MyAssetPack(pub Handle<Gltf>);

pub fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let handle: Handle<bevy::prelude::Scene> = ass.load("../assets/models/Test_map.gltf#Scene0");
    //commands.spawn_scene(handle);
    commands.insert_resource(IsLoaded(false));
    // let collider = ColliderBundle {
    //     shape: ColliderShape::cuboid(100.0, 0.1, 100.0).into(),
    //     ..Default::default()
    // };

    // commands
    //     .spawn_bundle(collider)
    //     .insert(ColliderPositionSync::Discrete)
    //     .insert(ColliderDebugRender::with_id(1));
}


pub fn spawn_gltf_objects(
    mut commands: Commands,
    q_parent: Query<&Children, With<MapModel>>,
    q_child: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    ass: Res<AssetServer>,
    mut is_loaded: ResMut<IsLoaded>,
) {
    let handle: Handle<bevy::prelude::Mesh> =
        ass.load("../assets/models/Test_map.gltf#Mesh0/Primitive0");
    let mesh = meshes.get(handle);

    if !mesh.is_none() && !is_loaded.0 {
        is_loaded.0 = true;
        //println!("mesh: {:?}", mesh);
        let scale = 251.44155883789062;
        commands.spawn()
        .insert(Collider::bevy_mesh(mesh.unwrap()).unwrap())
        .insert(Transform::from_scale(Vec3::new(scale,scale,scale)));


            
    }
}
