use crate::shared::resources::*;
use std::time::Duration;

use bevy::prelude::{shape::*, *};
use bevy::{
    input::mouse::MouseMotion,
    prelude::{KeyCode, *},
};
pub use bevy_prototype_debug_lines::*;
use super::resources::*;
pub use bevy_rapier3d::prelude::*;
use corgee::{additional::*, *, GoInputs, GoRot, GameState};

pub fn look<C: Component>(
    mut q_head: Query<(&Children, &mut Transform), (With<ZHead>, Without<Selected>)>,
    mut q_sel: Query<(&GoRot, &mut Transform, &Children), With<Selected>>,
    mut q_cam: Query<&mut Transform, (With<CharacterCamera>, Without<Selected>, Without<ZHead>)>,
    mut motion_evr: EventReader<MouseMotion>,
    sens: Res<Sensitivity>,
    time: Res<Time>
) {
    for (gorot, mut body_transform, children) in q_sel.iter_mut() {
        for &child in children.iter() {
            let (children, mut head_transform) = q_head.get_mut(child).unwrap();

            for &child in children.iter() {
                let mut cam_transform = q_cam.get_mut(child).unwrap();

                // body_transform.rotation = gorot.y;
                // cam_transform.rotation = gorot.x;
                for ev in motion_evr.iter() {
                    body_transform.rotation *= Quat::from_rotation_y(-ev.delta.x * sens.0 * 0.001);
                    cam_transform.rotation *= Quat::from_rotation_x(-ev.delta.y * sens.0 * 0.001);
                    //gorot.z = Quat::from_rotation_z(-ev.delta.x * SENSITIVITY); TODO!
                }

                //head_transform.rotation = gorot.z;

                //println!("gorot: {}, rb rotation: {}", gorot.x, body_transform.rotation);
            }
        }
    }
}

pub fn shoot<C: Component>(
    mut q_sel: Query<(&GoInputs, &Transform, &mut ShootTimer, &mut IsReadyShoot), With<C>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (ginp, transform, mut timer, mut can_shoot) in q_sel.iter_mut(){
        timer.0.tick(time.delta());
        if timer.0.finished(){
            can_shoot.0 = true;
        }
        if ginp.fire == 1 && can_shoot.0{
            can_shoot.0 = false;
            commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(Cube {
                    size: 0.05,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
                    alpha_mode: AlphaMode::Blend,
                    ..Default::default()
                }),
                transform: Transform{
                    translation: transform.translation + transform.forward() + Vec3::new(0., 0.4, 0.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Collider::ball(0.05))
            .insert(Velocity{
                linvel: transform.forward() * 20.,
                angvel: Vec3::ZERO,
            })
            
            .insert(GravityScale(1.))
            .insert(Damping {
                linear_damping: 0.4,
                angular_damping: 0.,
            })
            .insert(ColliderMassProperties::Density(2.5))
            .insert(Friction {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(RigidBody::Dynamic)
            .insert(ActiveEvents::COLLISION_EVENTS);

        }
    }
}
pub fn walk<C: Component>(
    mut q_sel: Query<
        (
            &GoInputs,
            &mut ExternalForce,
            &mut Velocity,
            &Transform,
            &MaxSpeed,
            &Acceleration,
        ),
        With<C>,
    >,
    time: Res<Time>,
) {
    for (ginp, mut force, mut velocity, transform, max_speed, acceleration) in q_sel.iter_mut()
    {
        let coef = time.delta_seconds() * acceleration.0 * 10.;

        let x = ginp.movement[0];
        let z = ginp.movement[1];

        let direction = transform.forward() * z + transform.right() * x;

        force.force = direction * coef;

        let speed = horizontal_speed(velocity.linvel);
        let flat_velocity = Vec3::new(velocity.linvel[0], 0., velocity.linvel[2]);
        if speed > max_speed.0 {
            let limited_vel = flat_velocity.normalize_or_zero() * max_speed.0;
            velocity.linvel = Vec3::new(limited_vel[0], velocity.linvel[1], limited_vel[2]);
        }

        // println!(
        //     "z: {}, x: {}, speed: {}, dir: {}",
        //     z,
        //     x,
        //     speed,
        //     direction.normalize_or_zero()
        // );
    }
}
pub fn is_grounded<C: Component>(
    mut q_sel: Query<(&Transform, Entity, &mut Damping), With<C>>,
    rapier_context: Res<RapierContext>,
    mut lines: ResMut<DebugLines>,
    mut commands: Commands,
    show_ray: Res<ShowRay>,
) {
    for (transform, ent, mut damping) in q_sel.iter_mut() {
        commands.entity(ent).remove::<Grounded>();
        damping.linear_damping = 0.5;

        let ray_pos = transform.translation;
        let ray_dir = Vec3::new(0., -1., 0.);
        let max_toi = 0.81;
        let groups = InteractionGroups::new(0b11, 0b1001).into();

        if let Some((_entity, toi)) =
            rapier_context.cast_ray(ray_pos, ray_dir, max_toi, false, groups)
        {
            damping.linear_damping = 10.;
            commands.entity(ent).insert(Grounded);

            if show_ray.0 {
                lines.line_colored(
                    ray_pos + ray_dir * toi,
                    ray_pos + ray_dir * max_toi,
                    0.0,
                    Color::MIDNIGHT_BLUE,
                );
            }
        }
        if show_ray.0 {
            lines.line_colored(ray_pos, ray_pos + ray_dir * max_toi, 0.0, Color::CYAN);
        }
    }
}
pub fn jump<C: Component>(
    mut q_sel: Query<(&GoInputs, &mut Velocity, &MaxJump), (With<C>, With<Grounded>)>,
) {
    for (inputs, mut vel, max_jump) in q_sel.iter_mut() {
        if inputs.jump == 1 {
            vel.linvel += Vec3::new(0., max_jump.0, 0.);
        }
    }
}
fn slab_handle() {}
fn sprint() {}
