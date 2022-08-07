mod characters;
//mod temp;
use bevy::prelude::*;
use reactive::*;
use characters::CharactersImpl;
use go_character::*;
use corgee::{character::*, *};
use go_level::plugin::Level;
use inspector::*;
 use ui::*;


fn main() {
    App::new()
        .add_plugin(Corgee)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(_temp_setup))
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(switch))
        .add_plugin(CharController)
        .add_plugin(CharactersImpl)
        .add_plugin(Level)
        .add_plugin(UI)
        .add_plugin(Inspector{game_version: env!("CARGO_PKG_VERSION")})
        .add_plugin(Reactive)        
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

    println!("setup");
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
    //spawner.send(SpawnChar("Soul", 1, -1));
    spawner.send(SpawnChar("Soul", 1, 1));

    selected.0 = Some(1);

}
