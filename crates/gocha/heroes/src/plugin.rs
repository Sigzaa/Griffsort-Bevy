use super::resources::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use core::prelude::{Character::*, GoInputs};

impl<T: Character<T> + Send + Sync + Copy> Plugin for CharPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type);
    }
}

impl<T: Character<T> + Send + Sync + Copy> CharPlugin<T> {
    pub fn new(char_type: T) -> Self {
        Self { char_type }
    }
}

pub trait Character<T: Character<T>>: Plugin {
    fn spawn<const CHAR_CODE: i32>(
        mut spawn_request: EventReader<SpawnCharacterRequest>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
    ) {
        for spawn_request in spawn_request.iter() {
            if spawn_request.1 == CHAR_CODE {
                commands
                    .spawn()
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
                    .insert(Core);
            }
        }
    }
}
