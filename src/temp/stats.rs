use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

use bevy::prelude::*;
use bevy::window::PresentMode;
//use reactive::*;
use super::super::characters::CharactersImpl;
use go_character::*;
use corgee::{character::*, *};



pub struct Stats;
impl Plugin for Stats {
    fn build(&self, app: &mut App) {
        app

       
            // .add_startup_system(fps)
            // .add_startup_system(tick)
            // .add_system(text_update_system)
            //.add_startup_system(example_startup_system)
            //.add_system(world::text_update_system)
            
            //.add_plugin(Console)
            ;
    }
}
fn example_startup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Load OBJ file
    let my_gltf = asset_server.load("models/just_a_girl/scene.gltf#Scene0");
    //models/adamHead/adamHead.gltf#Scene0
    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    let scale_factor = 0.01;
    commands.spawn_bundle(TransformBundle {
        local: Transform{
            translation: Vec3::ZERO,
            scale: Vec3::new(scale_factor, scale_factor,scale_factor),
            ..Default::default()
        },
        global: GlobalTransform::identity(),
    }).with_children(|parent| {
        parent.spawn_scene(my_gltf);
    });

    // let my_gltf = asset_server.load("models/wisp_forest/scene.gltf#Scene0");
    // //models/adamHead/adamHead.gltf#Scene0
    // // to be able to position our 3d model:
    // // spawn a parent entity with a TransformBundle
    // // and spawn our gltf as a scene under it
    // let scale_factor = 0.7;
    // commands.spawn_bundle(TransformBundle {
    //     local: Transform{
    //         translation: Vec3::ZERO,
    //         scale: Vec3::new(scale_factor, scale_factor,scale_factor),
    //         ..Default::default()
    //     },
    //     global: GlobalTransform::identity(),
    // }).with_children(|parent| {
    //     parent
    //     .spawn_scene(my_gltf)
        
    //     ;
    // })
    // .insert(RigidBody::Fixed)
    // .insert(Friction {
    //         coefficient: 4.5,
    //         combine_rule: CoefficientCombineRule::Min,
    //     })
    // //.insert(Collider::cuboid(100.0, 0., 100.0));
    // ;
    
}
#[derive(Component)]
struct TickDebug;
fn tick(
    mut commands: Commands, asset_server: Res<AssetServer>
){
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "Tick: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(TickDebug);
}
#[derive(Component)]
struct FpsText;
fn fps(
    mut commands: Commands, asset_server: Res<AssetServer>
){
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            // Use `Text` directly
            text: Text {
                // Construct a `Vec` of `TextSection`s
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                            font_size: 30.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FpsText);
}
use reactive::shared::resources::TickCount;
fn text_update_system(
    diagnostics: Res<Diagnostics>, 
    mut query: Query<&mut Text, (With<FpsText>, Without<TickDebug>)>,
    mut tick_q: Query<&mut Text, (With<TickDebug>, Without<FpsText>)>,
    tick_count: Res<TickCount>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                // Update the value of the second section
                text.sections[1].value = format!("{:.2}", average);
            }
        }
    }
    for mut text in tick_q.iter_mut() {

                text.sections[1].value = format!("{:.2}", tick_count.0);

    }
}