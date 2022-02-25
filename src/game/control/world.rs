use bevy::prelude::*;
use crate::game::components::{ filters::*, *};

pub fn spawn_teams(
    mut commands: Commands,
    asset_server: Res<super::AssetServer>,
    mut writer: EventWriter<SpawnCharacter>,
) {
    commands
    .spawn()
        .insert(ThreeDCam)
        .insert_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 1.2, -0.5),
            perspective_projection: PerspectiveProjection {
                fov: 1.9,
                ..Default::default()
            },
            ..Default::default()
        });
    println!("World file");
    writer.send(SpawnCharacter("Soul", 1,1));
    writer.send(SpawnCharacter("Root", 2,1));
    writer.send(SpawnCharacter("Root", 3,1));
    writer.send(SpawnCharacter("Soul", 4,-1));
    writer.send(SpawnCharacter("Root", 5,-1));
    writer.send(SpawnCharacter("Root", 6,-1));

    commands.insert_resource(BindedId(1));

    commands.spawn_bundle(UiCameraBundle::default());
    
        
    for _i in 1..8 {
            commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    "Not connected",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 20.0,
                        color: Color::AQUAMARINE,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        ..Default::default()
                    },
                ),
                ..Default::default()
            })
            .insert(HpText);
    }  
}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    
    commands.spawn_bundle(UiCameraBundle::default());
    let my_gltf = asset_server.load("../assets/circle.glb");

    commands.spawn_bundle((
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::identity(),
    )).with_children(|parent| {
        parent.spawn_scene(my_gltf);
    });
   
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
        material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            ..Default::default()
        }),
        ..Default::default()
    });
    // cube
    
    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.2,
    });
    
    const HALF_SIZE: f32 = 30.0;
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
                //scale: 0.1,
                ..Default::default()
            },
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
            ..Default::default()
        },
        ..Default::default()
    });

     
}

