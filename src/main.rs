mod characters;
mod temp;

use bevy::prelude::*;
use bevy::window::PresentMode;
//use go_multiplayer::*;
use characters::CharactersImpl;
use go_character::*;
use go_core::{Character::*, *};
use temp::stats::Stats;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Griffsort".to_string(),
            width: 720.,
            height: 500.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CharController)
        .add_plugin(CharactersImpl)
        .add_plugin(Core)
        .add_plugin(Stats)
        //.add_plugin(Networking)
        .add_startup_system(_temp_setup)
        .add_system(switch)
        .run();
}
fn switch(buttons: Res<Input<MouseButton>>, mut selected: ResMut<SelectedId>) {
    if buttons.just_pressed(MouseButton::Right) {
        let id = -selected.0.unwrap();
        selected.0 = Some(id);
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

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.5,
    });

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
            mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 0., 500.0));

    spawn_request.send(SpawnChar("Zero", 1, -1));
    spawn_request.send(SpawnChar("Zero", 1, 1));
    // spawn_request.send(SpawnChar("Soul", 1, 1));
    // spawn_request.send(SpawnChar("Soul", 1, 2));
    // spawn_request.send(SpawnChar("Soul", 1, 3));
    // spawn_request.send(SpawnChar("Soul", 1, 4));
    // spawn_request.send(SpawnChar("Soul", 1, 5)); // Spawning Soul in team 1 with id 0
    selected.0 = Some(1);
}
