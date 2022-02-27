use bevy::prelude::{*, };
use crate::game::components::{*, filters::*, player_data::*};
use heron::prelude::*;
#[derive(Component)]
pub struct Wall;


pub fn large_shield(
    mut commands: Commands,
    mut q_core: Query<(&Transform, &Control, &mut Timer1, &Team), With<SoulFilter>>,
    mut q_wall: Query<Entity, With<Wall>>,
    mut meshes: ResMut<super::Assets<Mesh>>,
    mut materials: ResMut<super::Assets<StandardMaterial>>,
    time: Res<Time>,
){  
    
    
    for (transform, ctrl, mut timer, team) in q_core.iter_mut(){
        
        if ctrl.q && timer.0 <= 0.{
            for entity in q_wall.iter_mut(){
                commands.entity(entity).despawn();
            }
            timer.0 = 2.;
            commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box { 
                    min_x: -4.,
                    max_x: 4.,
                    min_y: -0.4 ,
                    max_y: 3.,
                    min_z: -0.2,
                    max_z: 0.2,
                 })),
                 material: materials.add(StandardMaterial {
                    base_color: Color::rgba(227., 66., 52., 0.4),
                    alpha_mode: AlphaMode::Blend, 
                   ..Default::default()
                }),
                
                transform: transform.clone(),
                ..Default::default()
            })
            .insert(Hp(3200))
            .insert(Team(team.0))
            .insert(Counter(0.))
            .insert(Wall)
            .insert_bundle((
                CollisionShape::Cuboid { half_extends: Vec3::new(4.0,2.5,0.2) , border_radius: None}, // <-- A physics collision shape
                Transform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: transform.scale,
                }, // <-- Optionally define it's position
                GlobalTransform {
                    translation: transform.translation,
                    rotation: transform.rotation,
                    scale: transform.scale,
                },
            ))
            //.insert(CollisionShape::Cuboid { half_extends: Vec3::new(8.0,3.5,0.4) , border_radius: None})
            .insert(RigidBody::Sensor)
            ;      
        } else if timer.0 > 0. {
            println!("q could: {}", timer.0);
            timer.0 -= time.delta_seconds();
        }
    }
}
pub fn shield_deployment(
    mut q_wall: Query<(Entity, &mut Counter, &Transform), With<Wall>>,
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<super::Assets<Mesh>>,
    mut materials: ResMut<super::Assets<StandardMaterial>>,

){
    for (ent, mut counter, transform) in q_wall.iter_mut(){
        if counter.0 >= 0.3{
            return;
        }
        counter.0 += time.delta_seconds();
        commands.entity(ent).remove_bundle::<PbrBundle>();
        commands.entity(ent).insert_bundle(
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box { 
                    min_x: -4. * counter.0 * 3.,
                    max_x: 4. * counter.0 * 3.,
                    min_y: -0.4 * counter.0 * 3.,
                    max_y: 3. * counter.0 * 3.,
                    min_z: -0.2,
                    max_z: 0.2,
                 })),
                 material: materials.add(StandardMaterial {
                    base_color: Color::rgba(227., 66., 52., 0.4),
                    alpha_mode: AlphaMode::Blend, 
                   ..Default::default()
                }),
                
                transform: transform.clone(),
                ..Default::default()
            }
        );
    }
}