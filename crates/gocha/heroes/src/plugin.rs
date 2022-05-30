use super::resources::*;
use bevy::prelude::*;
use bevy::render::camera::ActiveCamera;
use bevy_rapier3d::prelude::*;
use core::prelude::{Character::*, GoInputs};

impl<T: Character<T> + Send + Sync + Copy + Component> Plugin for CharPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type)
            .add_event::<SpawnChar>()
            .add_system(T::spawn)
            .add_system(T::select)
            .add_system(T::extend::<T>);
    }
}

impl<T: Character<T> + Send + Sync + Copy> CharPlugin<T> {
    pub fn new(char_type: T) -> Self {
        Self { char_type }
    }
}

pub trait Character<T: Character<T>>: Plugin {
    fn extend<C: Component>(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        query: Query<Entity, Added<C>>,
    ) {
        for entity in query.iter() {
            println!("automatic extention has been worked");

            commands
                .entity(entity)
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.5 })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.7, 0.2, 0.3, 0.5),
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(2.0, 40.5, 15.0),
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
                .insert(Collider::ball(0.5))
                .insert(GravityScale(0.))
                .insert(GoInputs {
                    //forward: true,
                    ..Default::default()
                })
                .insert(Core)
                .with_children(|parent| {
                    // child cube
                    parent
                        .spawn_bundle(PerspectiveCameraBundle {
                            transform: Transform::from_xyz(0.0, 1.2, -0.5),
                            perspective_projection: PerspectiveProjection {
                                fov: 1.9,
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(Camera3d);
                });
        }
    }

    fn spawn(spawn_request: EventReader<SpawnChar>, commands: Commands);

    fn select(
        selected_id: Res<SelectedId>,
        sel: Query<&Id, With<Selected>>,
        q_core: Query<&Id>,
        q_camera: Query<(&Parent, Entity), With<Camera3d>>,
        mut commands: Commands,
        mut active_camera: ResMut<ActiveCamera<Camera3d>>,
    ) {
        for id in sel.iter() {
            if Some(id.0) == selected_id.0 {
                return;
            }
        }
        for (parent, ent) in q_camera.iter() {
            let id = q_core.get(parent.0).unwrap();
            if Some(id.0) == selected_id.0 {
                active_camera.set(ent);
            }
        }
    }
}
