use bevy::{
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};

use crate::game::components::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::prelude::nalgebra::Point3;

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
    commands.spawn_scene(handle);
    commands.insert_resource(IsLoaded(false));
    let collider = ColliderBundle {
        shape: ColliderShape::cuboid(100.0, 0.1, 100.0).into(),
        ..Default::default()
    };

    commands
        .spawn_bundle(collider)
        .insert(ColliderPositionSync::Discrete)
        .insert(ColliderDebugRender::with_id(1));
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
        let mesh = mesh.unwrap();
        is_loaded.0 = true;
        //println!("mesh: {:?}", mesh);
        let mut indices = mesh.indices().unwrap();

        let your_vertex_positions = match mesh.attribute("Vertex_Position").unwrap() {
            VertexAttributeValues::Float32x3(e) => e.clone(),
            _ => panic!("WHAT"),
        };
        let your_indices = match mesh.indices().unwrap() {
            Indices::U32(e) => e.clone(),
            _ => panic!("WHAT"),
        };
        /*
        let collider = ColliderBundle {
            shape: ColliderShape::trimesh(indices, vertices).into(),
            ..Default::default()
        };*/
        let mut points: Vec<Point3<f32>> = Vec::new();
        for vertex in your_vertex_positions {
            points.push(Point3::from_slice(&vertex) * 251.44155883789062);
        }
        // assert_eq!(0, indices.len() % 3);
        let mut indices = Vec::new();
        for i in 0..your_indices.len() {
            if i % 3 == 0 {
                indices.push([your_indices[i], your_indices[i + 1], your_indices[i + 2]]);
            }
        }
        let collider = ColliderBundle {
            shape: ColliderShape::trimesh(points, indices).into(),
            ..Default::default()
        };

        let scale_coef = 1.;
        commands.spawn_bundle((
            Transform::from_scale(Vec3::new(scale_coef, scale_coef, scale_coef)),
            GlobalTransform::identity(),
        )).with_children(|parent| {
            parent.spawn_bundle(collider)
            .insert(ColliderPositionSync::Discrete)
            .insert(ColliderDebugRender::with_id(1));
        });

            
    }
}
