mod characters;
mod temp;

use bevy::prelude::*;
use bevy::window::PresentMode;
use multigo::*;
use bevy_atmosphere::*;
use characters::CharactersImpl;
use go_character::*;
use corgee::{character::*, *};
use go_level::plugin::Level;
use temp::stats::Stats;


fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Griffsort".to_string(),
            width: 1200.,
            height: 800.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(SnapPlugin::<MySnap>::default())
        .insert_resource(Steps(3.))
        .insert_resource(bevy_atmosphere::AtmosphereMat::default()) // Default Earth sky
        .add_plugin(bevy_atmosphere::AtmospherePlugin {
            dynamic: false, // Set to false since we aren't changing the sky's appearance
            sky_radius: 50.0,
        })
        .add_plugin(CharController)
        .add_plugin(CharactersImpl)
        .add_plugin(Level)
        .add_plugin(Corgee)
        .add_plugin(Stats)
        .add_startup_system(_temp_setup)
        .add_system(add_id_provider)
        .add_system(switch)
        .add_system(save_keys)
        .add_system(store_snapshot)
        .add_plugin(Networking)
        .run();
}
fn switch(buttons: Res<Input<MouseButton>>, mut selected: ResMut<SelectedId>) {
    if buttons.just_pressed(MouseButton::Right) {
        let id = -selected.0.unwrap();
        selected.0 = Some(id);
    }
}
fn _temp_setup(
    mut spawner: EventWriter<SpawnChar>,
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
        .spawn()
        // .insert_bundle(PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Plane { size: 500.0 })),
        //     material: materials.add(StandardMaterial {
        //         base_color: Color::WHITE,
        //         perceptual_roughness: 1.0,
        //         ..default()
        //     }),
        //     ..default()
        // })
        .insert(RigidBody::Fixed)
        .insert(Friction {
            coefficient: 3.9,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Collider::cuboid(100.0, 0., 100.0));

    spawner.send(SpawnChar("Zero", 1, -1));
    //spawner.send(SpawnChar("Zero", 1, 1));

    selected.0 = Some(-1);
}
fn add_id_provider(
    q: Query<Entity, Added<ChCore>>,
    mut snapshot_id_provider: ResMut<SnapshotIdProvider<MySnap>>,
    mut commands: Commands,
){
    for ent in q.iter(){
        commands.entity(ent).insert(snapshot_id_provider.next());
    }
}

#[derive(Component, Reflect, Default)]
// Resources also (at least at the moment) need to be marked as Components, as well as Resources
#[reflect(Component, Resource)]
struct Steps(f32);

#[derive(Default, Debug)]
struct MySnap;

// Define types by creating a struct that implements the SnapType trait
impl SnapType for MySnap {
    fn add_types(registry: &mut TypeRegistry) {
        // Register the types you want to be saved and loaded
        registry.write().register::<Transform>();
        registry.write().register::<Steps>();


    }
}


fn save_keys(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::S) {
        info!("Making snapshot");
        // This triggers saving the world the next time commands are processed.
        // The snapshot is then sent as an event so it can be picked up by other systems.
        commands.save::<MySnap>();
    }
}

fn store_snapshot(
    mut save_events: EventReader<SaveEvent<MySnap>>,

) {
    for save_event in save_events.iter() {
        info!("Writing snapshot to save slot resource");

        // Save the snapshot in a resource so we can restore it later
        println!("{:?}",save_event.snapshot.clone());
    }
}
