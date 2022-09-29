mod characters_impl;
mod commands_impl;
pub mod heroes_mapping;

use actions::*;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_inspector_egui::{
    plugin::InspectorWindows, widgets::ResourceInspector, Inspectable, InspectorPlugin,
    WorldInspectorPlugin,
};
use bevy_rapier3d::prelude::*;
use characters_impl::{Characters, SpawnHeroEv};
use commands_impl::ConsoleCommands;
use go_level::plugin::Level;
use gs_inspector::*;
use gs_states::{
    CurrentState, CursorState, GameState, IntoConditionalSystem, KeyboardState, NextState,
    StatesPlugin,
};
use gs_ui::*;
use heroes::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use tokio::*;

use crate::characters_impl::{Jacqueline, Soul};

#[derive(Inspectable, Default)]
struct Data {
    clear_color: ResourceInspector<ClearColor>,
    ambient_light: ResourceInspector<AmbientLight>,
    acceleration: Config,
}

#[derive(Inspectable, Default)]
struct Acel(f32);

fn main() {
    App::new()
        .add_plugin(StatesPlugin)
        .insert_resource(WindowDescriptor {
            title: "griffsort".to_string(),
            width: 1320.,
            height: 600.,
            present_mode: PresentMode::Immediate,
            //mode: bevy::window::WindowMode::Fullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Inspector::new(env!("CARGO_PKG_VERSION")))
        .add_plugin(ConsoleCommands)
        .add_plugin(ActionsPlugin::<Action, Selected>::new(
            "./config/conf.ron",
            "./config/def.ron",
        ))
        .add_system(update_inputs::<Selected, Action>.before(collect_actions::<Selected, Action>))
        .add_enter_system(GameState::InGame, _temp_setup)
        .add_system(switch.run_in_state(GameState::InGame))
        .add_system(open_console)
        .add_plugin(CharController)
        .add_plugin(Characters)
        .add_plugin(Level)
        .add_plugin(UI)
        .run();
}

fn detect_action(
    q: Query<&Actions<Action>, With<Selected>>,
    mut bindings: ResMut<Keybindings<Action>>,
) {
    for act in &q
    {

        // act.debug();

        // if act.pressed(Action::Jump){
        //     bindings.mouse_bindings.insert(MouseButton::Middle, Action::Command(String::from("ha-ha its my perfect win, L")));
        // }
    }
}

#[derive(Serialize, Deserialize, Clone, Hash, PartialEq, Eq, Debug)]
pub enum Action {
    Shoot,
    Jump,
    Sprint,
    Left,
    Right,
    Back,
    Forward,
    Abil2,
    Abil1,
    Abil3,
    Ult,
    ToggleInspector,
    Command(String),
}

pub fn open_console(
    mut is_locked_actions: ResMut<IsLocked>,
    console: Res<ConsoleOpen>,
    mut commands: Commands,
) {
    if !console.is_changed()
    {
        return;
    }

    if console.open
    {
        commands.insert_resource(NextState(KeyboardState::Locked));
        commands.insert_resource(NextState(CursorState::Showed));
        is_locked_actions.0 = true;
    }
    else
    {
        commands.insert_resource(NextState(KeyboardState::Unlocked));
        commands.insert_resource(NextState(CursorState::Hided));
        is_locked_actions.0 = false;
    }
}

fn toggle_inspector(q: Query<&Actions<Action>>, mut inspector_windows: ResMut<InspectorWindows>) {
    for act in &q
    {
        if act.just_pressed(Action::ToggleInspector)
        {
            let mut inspector_window_data = inspector_windows.window_data_mut::<Data>();
            inspector_window_data.visible = !inspector_window_data.visible;
        }
    }
}

fn switch(
    buttons: Res<Input<MouseButton>>,
    mut selected: ResMut<SelectedId>,
    input: Res<Input<KeyCode>>,
) {
    if buttons.just_pressed(MouseButton::Middle)
    {
        let id = -selected.0.unwrap();
        selected.0 = Some(id);
    }
}

// fn test_new_inputs_system(
//     q: Query<&GoInputsNew<KeyCode>, With<Selected>>,
// ){
//     for ginp in &q{
//         let q = KeyCode::Q;
//         println!("q is: pressed: {}, just pressed {}, just released {}", ginp.pressed(q), ginp.just_pressed(q), ginp.just_released(q));
//     }
// }
fn _temp_setup(
    mut spawner: EventWriter<SpawnHeroEv>,
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
        .insert(Jacqueline);

    selected.0 = Some(0);
}

fn _masks_debug(query: Query<(Entity, &mut CollisionGroups)>) {
    for (ent, group) in query.iter()
    {
        println!(
            "entity: {:?} with membership: {:3.b}, filter: {:3.b},",
            ent, group.memberships, group.filters
        );
    }
}
