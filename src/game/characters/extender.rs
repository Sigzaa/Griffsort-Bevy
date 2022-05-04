// This module represents master for all <Core> entities.
// Any changes may brake entire game. Be careful.
use crate::game::components::{filters::*, player_data::*, *};
use bevy::prelude::*;

#[derive(Component)]
pub struct Extended;

pub fn extend(
    mut bind_event: EventReader<BindControls>,
    mut meshes: ResMut<super::Assets<Mesh>>,
    mut materials: ResMut<super::Assets<StandardMaterial>>,
    mut commands: Commands,
    mut q_camera: Query<Entity, With<ThreeDCam>>,
    q_core: Query<(Entity, &Id, &Team), (With<Core>, Without<Extended>)>,
) {
    for (ent, id, team) in q_core.iter() {
        let id = id.0;
        let team = team.0;


        let core = commands
            .entity(ent)
            .insert(Timer1(0.))
            //.insert(RigidBody::Dynamic)
            //.insert(RotationConstraints::lock())
            .insert(Extended)
            .insert(HeadRotation(Quat::from_rotation_y(0.)))
            //.insert(Veloctiy{
            //    translation: Vec3::new(0.,0.,0.),
            //})
            .insert(VelocityBuffer{
                linvel: Vec3::ZERO
            })
            .id();

        let head = commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.5 })),
                material: materials.add(StandardMaterial {
                    //base_color: Color::BLACK,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 1.2, -0.5),
                ..Default::default()
            })
            .insert(Head)
            .insert(Team(team as i16))
            .insert(Timer1(0.))
            .id();
        commands.entity(core).push_children(&[head]);

        for bind in bind_event.iter() {
            if id == bind.0 {
                println!("cam is binded");
                let cam = q_camera.single_mut();

                commands.entity(core).insert(Selected);
                commands.entity(core).push_children(&[cam]);
            }
        }
    }
}
