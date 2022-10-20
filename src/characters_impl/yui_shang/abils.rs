use bevy::prelude::{Query, With};
use heroes::CameraLink;

use crate::characters_impl::YuiShang;

use super::*;

pub fn shoot(
    mut q: Query<(&CameraLink, &mut ShootCD, &Actions<Action>), With<YuiShang>>,
    cam_q: Query<&GlobalTransform>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (cam_link, mut cd, act) in &mut q
    {
        cd.tick_timers(time.delta_seconds());

        if act.pressed(Action::Shoot) && cd.is_ready(0)
        {
            println!("poof");

            cd.add(-1);
            cd.cooldown(0, 0.5);


            let transform = cam_q.get(cam_link.0).unwrap().compute_transform();

            let mesh = Mesh::from(bevy::prelude::shape::Box::new(0.05, 0.05, 0.5,));
            let mut pos = transform.clone();
            pos.translation = pos.forward() * 1.5 + pos.translation;

            commands.spawn()
            .insert_bundle(PbrBundle{
                mesh: meshes.add(mesh.clone()),
                material: materials.add(StandardMaterial {
                    base_color: Color::DARK_GRAY,
                    metallic: 0.1,
                    ..Default::default()
                }),
                transform: pos,
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Velocity{
                linvel: transform.forward() * 100.,
                angvel: Vec3::ZERO
            })
            .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
            //.insert(Ccd::enabled())
            ;
            //println!("cd: {:?}", cd.0);


        }
        if cd.is_empty() || act.pressed(Action::Abil1)
        {
            cd.full(14);
            cd.cooldown(0, 2.);
        }
    }
}
