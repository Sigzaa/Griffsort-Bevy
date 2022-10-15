use bevy::{prelude::*, window::PresentMode};
use bevy_rapier3d::prelude::*;
use gs_states::*;
use heroes::{SelectedId, Team, Id, MaxHp};

use crate::characters_impl::{Jacqueline, SpawnHeroEv, Soul};

pub struct TempPlugin;

impl Plugin for TempPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowDescriptor {
            title: "griffsort".to_string(),
            width: 1320.,
            height: 600.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_enter_system(GameState::InGame, _temp_setup)
        .add_system(debug_component)
        .add_system(switch.run_in_state(GameState::InGame));
    }
}
fn debug_component(q: Query<(Entity, &MaxHp)>){
    for (ent, hp) in &q{
        //println!("ent :{ent:?}, hp: {}", hp.0);
    }
}
fn switch(
    buttons: Res<Input<MouseButton>>,
    mut selected: ResMut<SelectedId>,
    _input: Res<Input<KeyCode>>,
) {
    if buttons.just_pressed(MouseButton::Middle)
    {
        let id = -selected.0.unwrap();
        selected.0 = Some(id);
    }
}

fn _temp_setup(
    _spawner: EventWriter<SpawnHeroEv>,
    mut selected: ResMut<SelectedId>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("setup");
    const HALF_SIZE: f32 = 5.0;

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.6,
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
                scale: 1.,
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
        .spawn()
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Friction {
            coefficient: 3.9,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Collider::cuboid(100.0, 0., 100.0));
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });
    //spawner.send(SpawnHeroEv::new("Soul", Team::Light, -1));
    //spawner.send(SpawnHeroEv::new(String::from("jacqueline"), Team::Light, Vec3::new(15., 15., 15.,), 1));

    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(15., 15., 15.)),
        ))
        .insert(Jacqueline)
        .insert(Id(-1));

    commands
        .spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(Vec3::new(25., 15., 15.)),
        ))
        .insert(Team::Light)
        .insert(Jacqueline)
        .insert(Id(1));

    selected.0 = Some(1);
}
