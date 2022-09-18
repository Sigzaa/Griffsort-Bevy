mod characters;
use bevy::prelude::*;
use bevy::window::PresentMode;
use characters::CharactersImpl;
use go_level::plugin::Level;
use heroes::*;
use iyes_loopless::prelude::AppLooplessStateExt;
use std::hash::Hash;
// use gs_inspector::*;
use actions::*;
use bevy_inspector_egui::{
    plugin::InspectorWindows, widgets::ResourceInspector, Inspectable, InspectorPlugin,
    WorldInspectorPlugin,
};
use bevy_rapier3d::prelude::*;
use gs_states::{GameState, IntoConditionalSystem, StatesPlugin};
use gs_ui::*;
use serde::{Deserialize, Serialize};
use tokio::*;

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
        //.add_plugin(InspectorPlugin::<Keybindings<Action>>::new())
        .add_plugin(ActionsPlugin::<Action, Selected>::new(
            "./config/conf.ron",
            "./config/def.ron",
        ))
        .add_system(update_inputs::<Selected, Action>)
        //.add_system(collect_actions::<Selected, Action>)
        .add_system(detect_action)
        .add_enter_system(GameState::InGame, _temp_setup)
        // .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(_temp_setup))
        .add_system(switch.run_in_state(GameState::InGame))
        .add_plugin(CharController)
        .add_plugin(CharactersImpl)
        .add_plugin(Level)
        .add_plugin(UI)
        // .add_plugin(Inspector {
        //     game_version: env!("CARGO_PKG_VERSION"),
        // })
        // .add_plugin(AtmospherePlugin)
        //.add_system(test_new_inputs_system)
        //.add_plugin(Reactive)
        //.add_system(toggle_inspector)
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
    mut spawner: EventWriter<SpawnChar>,
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
    //spawner.send(SpawnChar::new("Soul", Team::Light, -1));
    spawner.send(SpawnChar::new("Soul", Team::Light, 1));

    selected.0 = Some(1);
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
