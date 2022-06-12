use super::resources::*;
use crate::shared::resources::*;

use bevy::prelude::{shape::*, *};
use bevy::render::camera::Camera3d;
use bevy::render::camera::{ActiveCamera, CameraTypePlugin};
use bevy_prototype_debug_lines::*;
pub use bevy_rapier3d::prelude::*;
use go_core::{additional::*, *, GoInputs, GoRot};

impl<T: Character<T> + Send + Sync + Copy + Component> Plugin for Controller<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type)
            .add_plugin(CameraTypePlugin::<CharacterCamera>::default())
            .add_event::<SpawnChar>()
            .add_system(T::spawn)
            .add_system(T::extend::<T>)
            .add_system(T::sync_components)
            .add_system(T::sync_camera)
            .add_system(T::is_grounded::<T>)
            .add_system(T::sync_rotation::<T>)
            //.add_system(T::float::<T>)
            .add_system(T::move_player::<T>)
            .add_system(T::jump::<T>)
            .add_system(T::shoot::<T>);
    }
}

pub trait Character<T: Character<T>>: Plugin {

    fn shoot<C: Component>(
        q_sel: Query<(&GoInputs, &Transform), With<C>>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        for (ginp, transform) in q_sel.iter(){
            if ginp.fire == 1{
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
                .insert(RigidBody::Dynamic);

            }
        }
    }
    fn move_player<C: Component>(
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
    fn is_grounded<C: Component>(
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
            let max_toi = 0.8;
            let groups = InteractionGroups::new(0b11, 0b1001);

            if let Some((_entity, toi)) =
                rapier_context.cast_ray(ray_pos, ray_dir, max_toi, false, groups, None)
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
    fn jump<C: Component>(
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

    fn extend<C: Component>(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        query: Query<(Entity, &Id), Added<C>>,
    ) {
        for (entity, id) in query.iter() {
            println!("automatic extention has been worked");

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
                    transform: Transform::from_xyz(2.0, 30., -id.0 as f32 * 1.5),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
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
                        .insert(ZHead)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(PerspectiveCameraBundle {
                                    transform: Transform::from_xyz(0., 0., 0.),
                                    perspective_projection: PerspectiveProjection {
                                        fov: 1.5,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(CharacterCamera);
                        });
                });

            commands
                .entity(entity)
                .insert(Collider::capsule(
                    Vec3::new(0., -0.4, 0.),
                    Vec3::new(0., 0.4, 0.),
                    0.4,
                ))
                .insert(CollisionGroups::new(0b10, 0b10))
                .insert(Velocity::default())
                .insert(ExternalForce::default())
                .insert(ExternalImpulse::default())
                .insert(GravityScale(2.))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Damping {
                    linear_damping: 6.5,
                    angular_damping: 0.,
                })
                .insert(ColliderMassProperties::Density(1.0))
                .insert(Friction {
                    coefficient: 1.,
                    combine_rule: CoefficientCombineRule::Min,
                })
                .insert(RigidBody::Dynamic);

            commands
                .entity(entity)
                .insert(RideHeight(0.4))
                .insert_bundle(Config::default())
                .insert(GoRot::default())
                .insert(GoInputs {
                    //jump: 1,
                    ..Default::default()
                })
                .insert(ChCore);
        }
    }
    fn sync_rotation<C: Component>(
        mut q_head: Query<(&Children, &mut Transform), (With<ZHead>, Without<Selected>)>,
        mut q_sel: Query<(&GoRot, &mut Transform, &Children), With<Selected>>,
        mut q_cam: Query<&mut Transform, (With<CharacterCamera>, Without<Selected>, Without<ZHead>)>,
    ) {
        for (gorot, mut body_transform, children) in q_sel.iter_mut() {
            for &child in children.iter() {
                let (children, mut head_transform) = q_head.get_mut(child).unwrap();

                for &child in children.iter() {
                    let mut cam_transform = q_cam.get_mut(child).unwrap();

                    body_transform.rotation = gorot.y;
                    cam_transform.rotation = gorot.x;
                    //head_transform.rotation = gorot.z;

                    //println!("gorot: {}, rb rotation: {}", gorot.x, body_transform.rotation);
                }
            }
        }
    }

    fn sync_camera(
        selected_id: Res<SelectedId>,
        q_camera: Query<Entity, With<CharacterCamera>>,
        q_head: Query<&Children, With<ZHead>>,
        q_core: Query<(&Id, &Children), (With<ChCore>, Without<Killed>, Without<Selected>)>,
        mut active_camera: ResMut<ActiveCamera<Camera3d>>,
    ) {
        for (id, children) in q_core.iter() {
            if Some(id.0) == selected_id.0 {
                for &child in children.iter() {
                    let children = q_head.get(child).unwrap();

                    for &child in children.iter() {
                        let cam_ent = q_camera.get(child).unwrap();
                        active_camera.set(cam_ent);
                    }
                }
            }
        }
    }

    fn sync_components(
        q_core: Query<(&Id, Entity), (With<ChCore>, Without<Killed>)>,
        selected_id: Res<SelectedId>,
        mut commands: Commands,
        active_camera: Res<ActiveCamera<Camera3d>>,
        q_camera: Query<Entity, With<CharacterCamera>>,
    ) {
        for (id, ent) in q_core.iter() {
            if Some(id.0) != selected_id.0 {
                commands.entity(ent).remove::<Selected>();
            } else {
                commands.entity(ent).insert(Selected);
            }
        }
        for ent in q_camera.iter(){
            commands.entity(ent).remove::<SelectedCamera>();
        }
        let cam_ent = active_camera.get();
        if  cam_ent != None{
            commands.entity(cam_ent.unwrap()).insert(SelectedCamera);
        }
        
    }
    fn spawn(spawn_request: EventReader<SpawnChar>, commands: Commands);
}

impl<T: Character<T> + Send + Sync + Copy> Controller<T> {
    pub fn new(char_type: T) -> Self {
        Self { char_type }
    }
}
