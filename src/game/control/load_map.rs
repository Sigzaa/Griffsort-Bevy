use bevy::{
    prelude::*
};

use bevy_rapier3d::prelude::*;
use bevy_rapier3d::geometry::ComputedColliderShape;

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



/// Helper resource for tracking our asset

pub fn load_gltf(mut commands: Commands, _ass: Res<AssetServer>) {
    //let handle: Handle<bevy::prelude::Scene> = ass.load("../assets/models/Test_map.gltf#Scene0");
    //commands.spawn_scene(handle);
    commands.insert_resource(IsLoaded(false));
}

pub fn spawn_gltf_objects(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    ass: Res<AssetServer>,
    mut is_loaded: ResMut<IsLoaded>,
) {
    let handle: Handle<bevy::prelude::Mesh> =
        ass.load("../assets/models/Test_map.gltf#Mesh0/Primitive0");
    let mesh = meshes.get(handle);

    if !mesh.is_none() && !is_loaded.0 {
        is_loaded.0 = true;
        let scale = 251.44155883789062;
        commands.spawn()
        .insert(Collider::from_bevy_mesh(mesh.unwrap(), &ComputedColliderShape::TriMesh).unwrap())
        .insert(Ccd::enabled())
        .insert(Transform::from_scale(Vec3::new(scale,scale,scale)));   
    }
}
