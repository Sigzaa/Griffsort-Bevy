use super::resources::*;
use bevy::prelude::*;
use bevy::render::camera::{ActiveCamera, CameraTypePlugin};
use bevy_config_cam::*;
use bevy_rapier3d::prelude::*;
use go_core::prelude::{Character::*, GoInputs};

impl<T: Character<T> + Send + Sync + Copy + Component> Plugin for CharPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type)
            .add_plugin(CameraTypePlugin::<CharacterCamera>::default())
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
                    mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.5 })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(2.0, 0.5, -id.0 as f32 * 1.5),
                    ..Default::default()
                })
                .insert_bundle(Config {
                    ..Default::default()
                })
                //.insert(RigidBody::Dynamic)
                // .insert(Velocity {
                //     linvel: Vec3::ZERO,
                //     angvel: Vec3::ZERO,
                // })
                .insert(LockedAxes::ROTATION_LOCKED)
                //.insert(Collider::ball(0.5))
                //.insert(GravityScale(0.))
                .insert(GoInputs {
                    //forward: true,
                    ..Default::default()
                })
                .insert(Core);
        }
    }

    fn spawn(spawn_request: EventReader<SpawnChar>, commands: Commands);

    fn select(
        selected_id: Res<SelectedId>,
        cam: Query<Entity, With<CharacterCamera>>,
        sel: Query<(Entity, &Id), With<Selected>>,
        q_core: Query<(&Id, Entity), (With<Core>, Without<Killed>)>,
        mut commands: Commands,
    ) {
        let cam_ent = match selected_id.0 {
            None => return,
            _ => cam.single(),
        };
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
}
