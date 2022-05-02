use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::game::components::{filters::*, player_data::*, *};
pub fn spawn(
    mut spawn_reader: EventReader<SpawnCharacter>,
    mut extend_writer: EventWriter<ExtendCharacter>,
    mut meshes: ResMut<super::Assets<Mesh>>,
    mut materials: ResMut<super::Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for spawn in spawn_reader.iter() {
        if spawn.0 == "Soul" {
            let id = spawn.1;
            let team = spawn.2;

            // let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_X
            //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z
            //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y;

            // let rigid_body = RigidBodyBundle {
            //     //body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
            //     position: Vec3::new(id as f32 * 2.0, 50.0, 0.0).into(),
            //     velocity: RigidBodyVelocity {
            //         linvel: Vec3::new(0.0, 0.0, 0.0).into(),
            //         angvel: Vec3::new(0.0, 0.0, 0.0).into(),
            //     }
            //     .into(),
            //     forces: RigidBodyForces {
            //         gravity_scale: 2.0,
            //         ..Default::default()
            //     }
            //     .into(),
            //     activation: RigidBodyActivation::cannot_sleep().into(),
            //     ccd: RigidBodyCcd {
            //         ccd_enabled: true,
            //         ..Default::default()
            //     }
            //     .into(),
            //     mass_properties: locked_dofs.into(),
            //     ..Default::default()
            // };
            // let collider = ColliderBundle {
            //     flags: ColliderFlags{
            //         collision_groups: InteractionGroups{
            //             filter: 2,
            //             memberships: 2,
            //         },
            //         ..Default::default()
                        
                    
            //     }.into(),
            //     shape: ColliderShape::ball(0.5).into(),
            //     material: ColliderMaterial {
            //         restitution: 0.,
            //         ..Default::default()
            //     }
            //     .into(),
            //     ..Default::default()
            // };

            let entity_id = commands
                .spawn()
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.5 })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.7, 0.2, 0.3, 0.5),
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(id as f32 * 2.0, 240.5, 15.0),
                    ..Default::default()
                })
                .insert(Spawn {
                    respawn_coords: Vec3::new(id as f32 * 2.0, 0.5, 15.0),
                })
                .insert_bundle(States {
                    character_name: CharName("Soul"),
                    team: Team(team as i16),
                    id: Id(id),
                    hor_vel: Speed(4.4),
                    hp: Hp(1000),
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Collider::ball(0.5))
                //.insert(ColliderPositionSync::Discrete)
                //.insert(ColliderDebugRender::with_id(0))
                .insert(Control {
                    //forward: true,
                    ..Default::default()
                })
                .insert(SoulFilter) // Change to Enum
                .insert(Core)
                //.insert(CollisionShape::Cuboid { half_extends: Vec3::new(0.5,0.5,0.5) , border_radius: None})
                .id();


            // let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_X
            //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z
            //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y;

            // let rigid_body = RigidBodyBundle {
            //     //dominance: RigidBodyDominance(1).into(),
            //     //body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
            //     position: Vec3::new(id as f32 * 10.0, 170.0, 0.0).into(),
            //     velocity: RigidBodyVelocity {
            //         linvel: Vec3::new(0.0, 0.0, 0.0).into(),
            //         angvel: Vec3::new(0.0, 0.0, 0.0).into(),
            //     }
            //     .into(),
            //     forces: RigidBodyForces {
            //         gravity_scale: 2.0,
            //         ..Default::default()
            //     }
            //     .into(),
            //     activation: RigidBodyActivation::cannot_sleep().into(),
            //     ccd: RigidBodyCcd {
            //         ccd_enabled: true,
            //         ..Default::default()
            //     }
            //     .into(),
            //     mass_properties: locked_dofs.into(),
            //     ..Default::default()
            // };
            // let collider = ColliderBundle {
            //     flags: ColliderFlags{
            //         collision_groups: InteractionGroups{
            //             filter: 1,
            //             memberships: 1,
            //         },
            //         ..Default::default()
                        
                    
            //     }.into(),
            //     shape: ColliderShape::ball(0.5).into(),
            //     material: ColliderMaterial {
            //         restitution: 0.,
            //         ..Default::default()
            //     }
            //     .into(),
            //     ..Default::default()
            // };
            
            // let backroll_id = commands
            //     .spawn()
            //     .insert_bundle(rigid_body)
            //     .insert_bundle(collider)
            //     .insert(ColliderPositionSync::Discrete)
            //     .insert(ColliderDebugRender::with_id(0))
            //     .insert(Id(id))
            //     .insert(Reconcile).id();

            extend_writer.send(ExtendCharacter(entity_id, id, team));
            //extend_writer.send(ExtendCharacter(backroll_id, id, team));
        }
    }
}
