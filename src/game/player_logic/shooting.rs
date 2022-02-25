use bevy::{ prelude::*};
use heron::prelude::*;
use crate::game::components::{bullet_states::*, filters::*, player_states::*, *};
pub struct Shooting;
impl Plugin for Shooting {
    fn build(&self, app: &mut App) {
        app
        .add_system(shoot_system)
        .add_system(continue_shoot_system)
        //.add_system(collision_system)
        .add_system(detect_collisions)
        ;
    }
}

fn detect_collisions(
    mut events: EventReader<CollisionEvent>,
    mut commands: Commands,
    q_bullet: Query<(Entity, &Dmg, &Team)>,
    mut q_obj: Query<(Entity, &mut Hp, &Team)>,
) {
    for event in events.iter() {
        match event {
            CollisionEvent::Started(data1, data2) => {
                let body1 = data1.rigid_body_entity();
                let body2 = data2.rigid_body_entity();
                let mut q1 = q_bullet.get(body1);
                let mut q2 = q_obj.get_mut(body2);
                if !q1.is_ok() || !q2.is_ok(){
                    if !q1.is_ok() && !q2.is_ok(){
                        q1 = q_bullet.get(body2);
                        q2 = q_obj.get_mut(body1);
                    } else {
                        return;
                    }
                }
                
                let (b_ent, dmg, b_team) = q1.unwrap();
                let (_obj_ent, mut hp, obj_team) = q2.unwrap();

                if b_team.0 != obj_team.0 {
                    commands.entity(b_ent).despawn();
                    hp.0 -= dmg.0;
                    //println!("{:?}, hp {:?}", player, hp.hp);
                    //commands.entity(player).despawn_recursive();
                }
            }
            CollisionEvent::Stopped(_data1, _data2) => {
                //println!("Entity {:?} and {:?} stopped to collide", data1.rigid_body_entity(), data2.rigid_body_entity())
            }
        }
    }
}

fn continue_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GlobalTransform, &BulletVelocity, &mut BulletLifeTime), With<Bullet>>,
) {
    for (entity, mut transform, velocity, mut life_time) in query.iter_mut() {
        if life_time.0 >= 0 {
            let direction = transform.forward();
            transform.translation += time.delta_seconds() * direction * velocity.0 as f32;
            life_time.0 -= velocity.0;
        } else {
            commands.entity(entity).despawn();
        }
    }
}
fn shoot_system(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut q_head: Query<(&Parent, &mut GlobalTransform, &Team, &mut Timer1), With<Head>>,
    q_core: Query<&Control>,
) {
    
    for (parent, transform, team, mut timer) in q_head.iter_mut() {
        let ctrl = q_core.get(parent.0).unwrap();
        if timer.0 <= 0. {
            if ctrl.lmb {
                timer.0 = 0.10;
                commands
                    .spawn()
                    .insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.08 })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::ORANGE,
                            ..Default::default()
                        }),
                        transform: Transform {
                            rotation: transform.rotation,
                            translation: transform.translation,
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Team(team.0))
                    .insert(Bullet)
                    .insert(BulletVelocity(25))
                    .insert(BulletLifeTime(10000))
                    .insert(CollisionShape::Cuboid { half_extends: Vec3::new(0.1,0.1,0.1) , border_radius: None})
                    .insert(RigidBody::Static)
                    .insert(Dmg(15));
            }
        } else {
            timer.0 -= time.delta_seconds();
        }
    }
}
