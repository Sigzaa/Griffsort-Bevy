use bevy::{
    prelude::{shape::*, *},
    render::camera::Projection,
};

use crate::HeroCam;

pub fn body(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entity: Entity,
) -> Entity {
    commands
        .entity(entity)
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(Capsule {
                radius: 0.3,
                ..Default::default()
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            //transform: *transform,
            ..Default::default()
        })
        .id()
}

pub fn head(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(Cube { size: 0.2 })),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GRAY,
                metallic: 0.1,
                ..Default::default()
            }),
            transform: Transform::from_xyz(0., 0.4, 0.),
            ..Default::default()
        })
        .id()
}
pub fn camera(commands: &mut Commands) -> Entity {
    commands
        .spawn_bundle(Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: 1.4, // a float of your fov in radians,
                ..default()
            }),
            camera: Camera {
                is_active: false,
                priority: 1,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 0., 0.),

            ..Default::default()
        })
        .insert(HeroCam)
        .id()
}
