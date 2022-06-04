use super::resources::*;
use bevy::prelude::*;
use bevy::render::camera::{ActiveCamera, CameraTypePlugin};
pub use bevy_rapier3d::prelude::*;
use go_core::{Character::*, GoInputs};

impl<T: Character<T> + Send + Sync + Copy + Component> Plugin for Controller<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type)
            .add_plugin(CameraTypePlugin::<CharacterCamera>::default())
            .add_event::<SpawnChar>()
            .add_system(T::spawn)
            //.add_system(T::sync_rotation)
            //.add_system(T::sync_camera)
            .add_system(T::extend::<T>)
            .add_system(T::hover::<T>);
    }
}

impl<T: Character<T> + Send + Sync + Copy> Controller<T> {
    pub fn new(char_type: T) -> Self {
        Self { char_type }
    }
}

pub trait Character<T: Character<T>>: Plugin {
    fn hover<C: Component>(
        rapier_context: Res<RapierContext>,
        mut q_character: Query<
            (
                &Transform,
                &mut ExternalForce,
                &mut ExternalImpulse,
                &Velocity,
            ),
            With<C>,
        >,
    ) {
        for (transform, mut ext_force, mut ext_impulse, velocity) in q_character.iter_mut() {
            let ray_pos = transform.translation;
            let ray_dir = ray_pos - Vec3::new(0., 0.5, 0.);
            let max_toi = 30.0;
            let solid = false;
            let groups = InteractionGroups::all();
            let filter = None;

            if let Some((entity, toi)) =
                rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, groups, filter)
            {
                // The first collider hit has the entity `entity` and it hit after
                // the ray travelled a distance equal to `ray_dir * toi`.
                //ext_force.force = Vec3::new(0., 4. / toi, 0.);
                //ext_force.torque = Vec3::new(100., 20., 0.);
                let ride_height = 0.65;
                let x = toi - ride_height;
                let spring_force = (x * 1.) - ( velocity.linvel[1] * 1.);

                ext_force.force = Vec3::new(0., spring_force, 0.) * ray_dir;


                let hit_point = ray_pos + ray_dir * toi;
                //println!("vel {} toi {}", velocity.linvel, toi);
            }
        }
    }

    fn extend<C: Component>(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        query: Query<(Entity, &Id), Added<C>>,
    ) {
        for (entity, id) in query.iter() {
            println!("automatic extention has been worked");

            // // back (right) wall
            // let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
            // transform.rotate(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
            // commands.spawn_bundle(PbrBundle {
            //     mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
            //     transform,
            //     material: materials.add(StandardMaterial {
            //         base_color: Color::INDIGO,
            //         perceptual_roughness: 1.0,
            //         ..default()
            //     }),
            //     ..default()
            // });

            // ambient light
            commands.insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 0.5,
            });

            commands
                .entity(entity)
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(bevy::prelude::shape::Capsule {
                        radius: 0.3,
                        ..Default::default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(2.0, 5.5, -id.0 as f32 * 1.5),
                    ..Default::default()
                })
                .insert_bundle(Config {
                    ..Default::default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Velocity {
                    linvel: Vec3::ZERO,
                    angvel: Vec3::ZERO,
                })
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Collider::capsule(
                    Vec3::new(0., -0.4, 0.),
                    Vec3::new(0., 0.4, 0.),
                    0.4,
                ))
                .insert(ExternalForce {
                    force: Vec3::ZERO,
                    torque: Vec3::ZERO,
                })
                .insert(ExternalImpulse {
                    impulse: Vec3::new(1.0, 2.0, 3.0),
                    torque_impulse: Vec3::new(0.1, 0.2, 0.3),
                })
                .insert(GravityScale(2.))
                .insert(GoInputs {
                    //forward: true,
                    ..Default::default()
                })
                .insert(ChCore)
                .with_children(|parent| {
                    // child cube
                    parent
                        .spawn()
                        .insert(CharacterCamera)
                        .insert_bundle(PerspectiveCameraBundle {
                            transform: Transform::from_xyz(-2.0, 2.5, 5.0)
                                .looking_at(Vec3::ZERO, Vec3::new(0., 0.5, -id.0 as f32 * 1.5)),
                            perspective_projection: PerspectiveProjection {
                                fov: 1.0,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                });
        }
    }

    fn spawn(spawn_request: EventReader<SpawnChar>, commands: Commands);

    fn sync_camera(
        selected_id: Res<SelectedId>,
        cam: Query<Entity, With<CharacterCamera>>,
        sel: Query<(Entity, &Id), With<Selected>>,
        q_core: Query<(&Id, Entity), (With<ChCore>, Without<Killed>)>,
        mut commands: Commands,
    ) {
        let cam_ent = cam.single();

        for (sel_ent, id) in sel.iter() {
            if Some(id.0) == selected_id.0 {
                return;
            }
            commands.entity(sel_ent).remove::<Selected>();
            commands.entity(sel_ent).remove_children(&[cam_ent]);
        }
        for (id, ent) in q_core.iter() {
            if Some(id.0) == selected_id.0 {
                commands.entity(ent).push_children(&[cam_ent]);
                commands.entity(ent).insert(Selected);
            }
        }
    }

    fn sync_rotation() {}
}
