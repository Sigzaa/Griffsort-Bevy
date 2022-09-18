use super::resources::*;
use bevy::prelude::{shape::*, *};
use bevy::reflect::TypeUuid;
use bevy::render::camera::Projection;
use bevy::{
    input::mouse::MouseMotion,
    prelude::{KeyCode, *},
};
use bevy_atmosphere::prelude::*;
use bevy_rapier3d::prelude::*;
use std::time::Duration;
// use bevy::render::camera::Camera3dBundle;
// use bevy::render::camera::{ActiveCamera, CameraTypePlugin};
use actions::*;
use bevy_prototype_debug_lines::*;

#[derive(Default)]
pub struct Controller<T: 'static> {
    pub char_type: T,
}

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
        for (entity, id) in query.iter()
        {
            let head = commands
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
                .id();

            let camera = commands
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
                .insert(HeroCam)
                .insert(AtmosphereCamera(None))
                .id();

            let body = commands
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
                .insert(HeadLink(head))
                .insert(CameraLink(camera))
                .id();

            commands.entity(body).push_children(&[head]);
            commands.entity(head).push_children(&[camera]);

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
                .insert(Hero)
                .insert(PointingOn::default());
        }
    }

    fn sync_camera(
        selected_id: Res<SelectedId>,
        mut q_camera: Query<&mut Camera>,
        q_core: Query<(&Id, &CameraLink), (With<Hero>, Without<Dead>)>,
    ) {
        if selected_id.is_changed()
        {
            for (id, cam_link) in q_core.iter()
            {
                let mut camera = q_camera.get_mut(cam_link.0).unwrap();

                camera.is_active = false;

                if Some(id.0) == selected_id.0
                {
                    camera.is_active = true;
                }
            }
        }
    }

    fn sync_components(
        q_core: Query<(&Id, Entity), (With<Hero>, Without<Dead>)>,
        selected_id: Res<SelectedId>,
        mut commands: Commands,
        //active_camera: Res<ActiveCamera<Camera3d>>,
        q_camera: Query<Entity, With<HeroCam>>,
    ) {
        for (id, ent) in q_core.iter()
        {
            if Some(id.0) != selected_id.0
            {
                commands.entity(ent).remove::<Selected>();
            }
            else
            {
                commands.entity(ent).insert(Selected);
            }
        }
        for ent in q_camera.iter()
        {
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
