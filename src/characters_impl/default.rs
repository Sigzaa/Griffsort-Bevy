use actions::Actions;
use bevy::prelude::{*};
use bevy::{
    input::mouse::MouseMotion,
};
use heroes::*;



use crate::Action;

pub fn sync_configs() {
    // We have Config in components and the same Config in resources
    // We can read both, but we can onl
    // Syncing Heroes::Config
}

pub fn look<C: Component>(
    mut hero_q: Query<(&CameraLink, &mut Transform), (With<C>, With<Selected>)>,
    mut q_cam: Query<&mut Transform, Without<C>>,
    mut motion_evr: EventReader<MouseMotion>,
    conf: Res<HeroesConfig>,
) {
    for (cam_link, mut body_transform) in hero_q.iter_mut()
    {
        //println!("pos: {}", body_transform.translation);
        let mut cam_transform = q_cam.get_mut(cam_link.0).unwrap();

        for ev in motion_evr.iter()
        {
            body_transform.rotation *=
                Quat::from_rotation_y(-ev.delta.x * conf.sensitivity * 0.001);
            cam_transform.rotation *= Quat::from_rotation_x(-ev.delta.y * conf.sensitivity * 0.001);
        }
    }
}

pub fn pointing_on_shape<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    _global_conf: Res<HeroesConfig>,
    conf: Res<Conf>,
    rapier_context: Res<RapierContext>,
    mut hero_q: Query<(&CameraLink, &mut ShapeIntersections), With<C>>,
    q_cam: Query<&GlobalTransform>,
    _commands: Commands,
) {
    for (camera_entity, mut pointing_on) in &mut hero_q
    {
        pointing_on.0.clear();

        let cam_transform = q_cam.get(camera_entity.0).unwrap();
        let (_, c_rotation, _) = cam_transform.to_scale_rotation_translation();

        rapier_context.intersections_with_shape(
            cam_transform.translation()
                + cam_transform.forward() * conf.props().pointing_shape.source_distance,
            c_rotation,
            &Collider::cylinder(
                conf.props().pointing_shape.radius.into(),
                conf.props().pointing_shape.toi,
            ),
            QueryFilter::only_dynamic(),
            |entity| {
                pointing_on.0.push(entity);
                true // Return `false` instead if we want to stop searching for other colliders that contain this point.
            },
        );

        // println!("pointing: {:?}", pointing_on.0);
        // println!();
    }
}

pub fn pointing_on<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    global_conf: Res<HeroesConfig>,
    conf: Res<Conf>,
    rapier_context: Res<RapierContext>,
    mut hero_q: Query<(&CameraLink, &mut RayPointingOn), With<C>>,
    q_cam: Query<&GlobalTransform>,
    mut lines: ResMut<DebugLines>,
) {
    for (camera_entity, mut pointing_on) in &mut hero_q
    {
        let cam_transform = q_cam.get(camera_entity.0).unwrap();

        let start = cam_transform.translation() + cam_transform.forward() * 1.5;

        let mut max_toi = conf.props().pointing_ray_toi;

        if let Some((entity, toi)) = rapier_context.cast_ray(
            start,
            cam_transform.forward(),
            max_toi,
            true,
            QueryFilter::only_dynamic(),
        )
        {
            max_toi = toi;
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.

            pointing_on.0 = Some((entity, toi));
        }
        else
        {
            pointing_on.0 = None;
        }

        if global_conf.showray
        {
            lines.line_colored(
                start,
                cam_transform.translation() + cam_transform.forward() * max_toi,
                0.,
                global_conf.ray_color,
            );
        }
    }
}

// pub fn fly<C: Component>(mut q_sel: Query<
//     (
//         &GoInputs,
//         &mut ExternalForce,
//         &mut Velocity,
//         &Transform,
//         &MaxSpeed,
//         &Acceleration,
//         &NoClip
//     ),
//     With<C>,
// >,
// time: Res<Time>,){
//     for (ginp, mut force, mut velocity, transform, max_speed, acceleration, noclip) in q_sel.iter_mut()
//     {
//         let mut coef = 0.;
//         if ginp.jump() && noclip.0{
//             coef = time.delta_seconds() * acceleration.0 * 10.;
//         }
//         if ginp.esc() && noclip.0{
//             coef = -time.delta_seconds() * acceleration.0 * 10.;
//         }

//         velocity.linvel += Vec3::new(0., coef, 0.) * 0.01;
//     }

// }
// pub fn noclip_handler(mut q: Query<(&mut GravityScale, &NoClip, &mut MaxSpeed), Changed<NoClip>>){
//     for (mut grav_scale, is_noclip, mut max_speed) in q.iter_mut() {
//         if is_noclip.0
//         {
//             grav_scale.0 = 0.;
//             //max_speed.0 += 44.;
//         }
//         else
//         {
//             grav_scale.0 = 2.;
//             //max_speed.0 -= 44.;
//         }

//     }
// }

// pub fn shoot<C: Component>(
//     mut q_sel: Query<(&GoInputs, &Transform, &mut ShootTimer, &mut IsReadyShoot), With<C>>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     time: Res<Time>,
// ) {
//     for (ginp, transform, mut timer, mut can_shoot) in q_sel.iter_mut() {
//         timer.0.tick(time.delta());
//         if timer.0.finished() {
//             can_shoot.0 = true;
//         }
//         if ginp.shoot == 1 && can_shoot.0 {
//             can_shoot.0 = false;
//             commands
//                 .spawn_bundle(PbrBundle {
//                     mesh: meshes.add(Mesh::from(Cube { size: 0.05 })),
//                     material: materials.add(StandardMaterial {
//                         base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
//                         alpha_mode: AlphaMode::Blend,
//                         ..Default::default()
//                     }),
//                     transform: Transform {
//                         translation: transform.translation
//                             + transform.forward()
//                             + Vec3::new(0., 0.4, 0.),
//                         ..Default::default()
//                     },
//                     ..Default::default()
//                 })
//                 .insert(Collider::ball(0.05))
//                 .insert(Velocity {
//                     linvel: transform.forward() * 20.,
//                     angvel: Vec3::ZERO,
//                 })
//                 .insert(GravityScale(1.))
//                 .insert(Damping {
//                     linear_damping: 0.4,
//                     angular_damping: 0.,
//                 })
//                 .insert(ColliderMassProperties::Density(2.5))
//                 .insert(Friction {
//                     coefficient: 1.,
//                     combine_rule: CoefficientCombineRule::Min,
//                 })
//                 .insert(RigidBody::Dynamic)
//                 .insert(ActiveEvents::COLLISION_EVENTS);
//         }
//     }
// // }
pub fn walk<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut q_sel: Query<
        (
            &Actions<Action>,
            &mut ExternalForce,
            &mut Velocity,
            &Transform,
        ),
        With<C>,
    >,
    conf: Res<Conf>,
    time: Res<Time>,
) {
    for (ginp, mut force, mut velocity, transform) in q_sel.iter_mut()
    {
        let (right, forward) =
            ginp.cross(Action::Left, Action::Right, Action::Back, Action::Forward);

        let direction = transform.forward() * forward + transform.right() * right;

        let props = conf.props();
        let coef = time.delta_seconds() * props.acceleration * 10.;

        force.force = direction * coef;

        let speed = horizontal_speed(velocity.linvel);

        let flat_velocity = Vec3::new(velocity.linvel[0], 0., velocity.linvel[2]);

        // println!(
        //     "vel before: {flat_velocity}, vel after: {}",
        //     flat_velocity.clone().normalize_or_zero()
        // );

        if speed > props.max_velocity * 0.3
        {
            let limited_vel = flat_velocity.normalize_or_zero() * props.max_velocity * 0.3;
            velocity.linvel = Vec3::new(limited_vel[0], velocity.linvel[1], limited_vel[2]);
        }

        // println!("speed: {speed:.2}");
    }
}

#[derive(Default)]
pub struct SinIn(pub f32);

pub fn camera_shake<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    conf: Res<Conf>,
    q_sel: Query<(&HeadLink, &Actions<Action>), With<C>>,
    mut head: Query<&mut Transform>,
    mut sin_input: Local<SinIn>,
    time: Res<Time>,
) {
    for (link, act) in &q_sel
    {
        let mut head_transform = head.get_mut(link.0).unwrap();

        if act.pressed(Action::Left)
            || act.pressed(Action::Right)
            || act.pressed(Action::Back)
            || act.pressed(Action::Forward)
        {
            head_transform.translation =
                Vec3::new(0., sin_input.0.sin() * conf.props().camera.shake_ampl, 0.);
            sin_input.0 += time.delta_seconds() * conf.props().camera.shake_rate;
        }
        else
        {
            sin_input.0 = 0.;
        }
    }
}

pub fn camera_roll<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut q_cam: Query<&mut Transform>,
    mut q_sel: Query<&CameraLink, (With<Selected>, With<C>)>,
    _conf: Res<Conf>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    for camera_link in &mut q_sel
    {
        let _cam_transform = q_cam.get_mut(camera_link.0).unwrap();

        let mut is_idle = true;

        // cam_transform.rotation = Quat::from_rotation_z(15.);

        // println!("camera transform: {:?}", cam_transform.rotation.to_scaled_axis());

        for _ev in motion_evr.iter()
        {
            // Mouse is moving

            // cam_transform.rotation = ease(cam_transform.rotation.wrap(), Quat::from_rotation_z(out * max_roll).wrap());

            is_idle = false;
        }
        if is_idle
        {
            // Mouse not moving

            //val_to(&mut func_input.0, 0., time.delta_seconds() * out_time);
        }
        //head_transform.rotation = ;

        //println!("in: {:.4}", func_input.0);
    }
}

pub fn is_grounded<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut q_sel: Query<(&Transform, Entity, &mut Damping), With<C>>,
    rapier_context: Res<RapierContext>,
    _lines: ResMut<DebugLines>,
    mut commands: Commands,
    //show_ray: Res<ShowRay>,
) {
    for (transform, ent, _damping) in q_sel.iter_mut()
    {
        //
        // damping.linear_damping = 0.5;

        let shape = Collider::ball(0.1);
        let shape_pos = transform.translation;
        let shape_rot = transform.rotation;
        let shape_vel = transform.down() + Vec3::new(0., -1., 0.);
        let max_toi = 0.4;
        let filter = QueryFilter::new().exclude_collider(ent);

        if let Some((_entity, _hit)) =
            rapier_context.cast_shape(shape_pos, shape_rot, shape_vel, &shape, max_toi, filter)
        {
            // The first collider hit has the entity `entity`. The `hit` is a
            // structure containing details about the hit configuration.
            commands.entity(ent).insert(Grounded);
        }
        else
        {
            commands.entity(ent).remove::<Grounded>();
        }
    }
}
pub fn jump<C: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut q_sel: Query<(&Actions<Action>, &mut Velocity), (With<C>, With<Grounded>)>,
    conf: Res<Conf>,
) {
    for (inputs, mut vel) in q_sel.iter_mut()
    {
        if inputs.just_pressed(Action::Jump)
        {
            vel.linvel += Vec3::new(0., conf.props().max_jump_height, 0.);
        }
    }
}
