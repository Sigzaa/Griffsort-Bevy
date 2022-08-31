use super::resources::*;
use bevy::prelude::{shape::*, *};
use bevy::reflect::TypeUuid;
use bevy::render::camera::Projection;
use bevy::{
    input::mouse::MouseMotion,
    prelude::{KeyCode, *},
};
use bevy_atmosphere::prelude::*;
use std::time::Duration;
// use bevy::render::camera::Camera3dBundle;
// use bevy::render::camera::{ActiveCamera, CameraTypePlugin};
use bevy_prototype_debug_lines::*;
use corgee::{additional::*, GameState, GoInputs, GoRot, *};

impl<T: Character<T> + Send + Sync + Copy + Component> Plugin for Controller<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type)
            .add_event::<SpawnChar>()
            .add_system(T::spawn)
            .add_system(T::extend::<T>)
            .add_system(T::sync_components)
            .add_system(T::sync_camera);
    }
}

pub trait Character<T: Character<T>>: Plugin {
    fn extend<C: Component>(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        query: Query<(Entity, &Id), Added<C>>,
    ) {
        for (entity, id) in query.iter() {
            println!("Automatic extention has been worked");

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
                    transform: Transform::from_xyz(-24.0, 21., (-id.0 as f32 * 1.5) + 50.),
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
                                .spawn_bundle(Camera3dBundle {
                                    projection: Projection::Perspective(PerspectiveProjection {
                                        fov: 1.4, // a float of your fov in radians,
                                        ..default()
                                    }),
                                    camera: Camera {
                                        is_active: true,
                                        priority: id.0 as isize,
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(0., 0., 0.),

                                    ..Default::default()
                                })
                                .insert(CharacterCamera)
                                .insert(AtmosphereCamera(None));
                        });
                });

            commands
                .entity(entity)
                .insert_bundle(bevy_mod_picking::PickableBundle::default())
                .insert(Collider::capsule(
                    Vec3::new(0., -0.4, 0.),
                    Vec3::new(0., 0.4, 0.),
                    0.4,
                ))
                .insert(Velocity::default())
                .insert(ExternalForce::default())
                .insert(ExternalImpulse::default())
                .insert(GravityScale(2.))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(Damping {
                    linear_damping: 1.5,
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
                .insert(ChCore)
                .insert(ShootTimer(Timer::new(Duration::from_secs(1), true)))
                .insert(IsReadyShoot(true))
                .insert(QTimer(0.))
                .insert(ETimer(0.))
                .insert(FTimer(0.))
                .insert(ShiftTimer(0.));
        }
    }

    fn sync_camera(
        selected_id: Res<SelectedId>,
        //q_camera: Query<Camera>,
        q_head: Query<&Children, With<ZHead>>,
        q_core: Query<(&Id, &Children), (With<ChCore>, Without<Killed>, Without<Selected>)>,
        //mut active_camera: ResMut<ActiveCamera<Camera3d>>,
    ) {
        for (id, children) in q_core.iter() {
            if Some(id.0) == selected_id.0 {
                for &child in children.iter() {
                    let children = q_head.get(child).unwrap();

                    // for &child in children.iter() {
                    //     let cam_ent = q_camera.get(child).unwrap();
                    //     //active_camera.set(cam_ent);
                    // }
                }
            }
        }
    }

    fn sync_components(
        q_core: Query<(&Id, Entity), (With<ChCore>, Without<Killed>)>,
        selected_id: Res<SelectedId>,
        mut commands: Commands,
        //active_camera: Res<ActiveCamera<Camera3d>>,
        q_camera: Query<Entity, With<CharacterCamera>>,
    ) {
        for (id, ent) in q_core.iter() {
            if Some(id.0) != selected_id.0 {
                commands.entity(ent).remove::<Selected>();
            } else {
                commands.entity(ent).insert(Selected);
            }
        }
        for ent in q_camera.iter() {
            commands.entity(ent).remove::<SelectedCamera>();
        }
        // let cam_ent = active_camera.get();
        // if  cam_ent != None{
        //     commands.entity(cam_ent.unwrap()).insert(SelectedCamera);
        // }
    }
    fn spawn(spawn_request: EventReader<SpawnChar>, commands: Commands);
}

impl<T: Character<T> + Send + Sync + Copy> Controller<T> {
    pub fn new(char_type: T) -> Self {
        Self { char_type }
    }
}
