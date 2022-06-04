mod default_char;
mod soul;

use go_character::*;
use go_core::prelude::Character::*;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

#[derive(Copy, Clone, Component)]
struct Soul;

#[derive(Copy, Clone, Component)]
struct DefaultChar;

pub struct CharactersImpl;
impl Plugin for CharactersImpl {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(_temp_setup)
            .add_plugin(Controller::<DefaultChar>::new(DefaultChar))
            .add_plugin(Controller::<Soul>::new(Soul));
    }
}

fn _temp_setup(
    mut spawn_request: EventWriter<SpawnChar>,
    mut selected: ResMut<SelectedId>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    const HALF_SIZE: f32 = 10.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..default()
        },
        ..default()
    });

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(5.0, 0., 5.0));

    spawn_request.send(SpawnChar("Soul", 1, 0));
    // spawn_request.send(SpawnChar("Soul", 1, 1));
    // spawn_request.send(SpawnChar("Soul", 1, 2));
    // spawn_request.send(SpawnChar("Soul", 1, 3));
    // spawn_request.send(SpawnChar("Soul", 1, 4));
    // spawn_request.send(SpawnChar("Soul", 1, 5)); // Spawning Soul in team 1 with id 0
    // selected.0 = Some(5);
}
