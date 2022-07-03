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
        .add_plugins(DefaultPlugins)
        .add_system_set(SystemSet::on_enter(GameState::InGame).with_system(_temp_setup))
        .add_system_set(SystemSet::on_update(GameState::InGame).with_system(switch))
        .add_plugin(CharController)
        .add_plugin(CharactersImpl)
        .add_plugin(Level)
        
        .add_plugin(UI)
        .add_plugin(Inspector)
        .add_plugin(Reactive)

        
        // .add_plugin(bevy_atmosphere::AtmospherePlugin {
        //     dynamic: false, // Set to false since we aren't changing the sky's appearance
        //     sky_radius: 50.0,
        // })
                //.add_plugin(SnapPlugin::<MySnap>::default())
        //.insert_resource(Steps(3.))
        //  // Default Earth sky
        //.add_startup_system(_temp_setup)
        // .add_system(store_snapshot)
        // .add_system(save_keys)
        //.add_system(add_id_provider)
        //.add_system(switch)
        //.add_system(save_keys)
        //.add_system(store_snapshot)
        // .add_plugin(SnapPlugin::<HeySnap>::default())
        // .add_startup_system(startup)
        //.add_startup_system(sync_types)
        
        //.add_system(simple.exclusive_system().with_run_criteria(bevy::core::FixedTimestep::step(1.)))
        
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
    spawner.send(SpawnChar("Zero", 1, -1));
    spawner.send(SpawnChar("Zero", 1, 1));

    selected.0 = Some(-1);
}

// #[derive(Default, Debug)]
// struct HeySnap;

// #[derive(Component, Reflect, Default)]
// // And be marked as components
// #[reflect(Component)]
// struct Com;

// impl SnapType for HeySnap {
//     fn add_types(registry: &mut TypeRegistry) {
//         // Register the types you want to be saved and loaded
//         registry.write().register::<Transform>();
//         registry.write().register::<Com>();

//     }
// }


// fn startup(
//     mut snapshot_id_provider: ResMut<SnapshotIdProvider<HeySnap>>,
//     q: Query<Entity, With<ChCore>>,
//     mut commands: Commands,
// ){
//     for ent in q.iter(){
//         commands.entity(ent).insert(snapshot_id_provider.next());
//     }
// }
// // fn sync_types(snap: ResMut<SnapServer>){
// //     snap.types
// //     .res::<Transform>()
// //     .com::<Hp>()
// //     .add::<Velocity>()
// //     .add::<Selected>();
// // }

// // fn sync_types(snap: ResMut<SnapServer>){
// //     snap.types
// //     .add::<Query<(&Velocity, &Hp), With<NetSync>>>();
// // }


// type Net = (Velocity, Hp);
// fn make_snap(world: &mut World){
//     //let mut q = world.query_filtered::<Entity, With<Selected>>();
//     let snap = world.get_resource::<SnapServer>().unwrap();

//     // for ent in q.iter(&world){
//     //     snap.shot::<Velocity>(world, ent);
//     // }
    
// }
// struct Hi;

// struct MyTypes;


// trait Networking{}
// struct Test;


// use bevy::ecs::storage::SparseSets;

// impl Networking for Id{}
// impl Networking for Transform{}
// impl Networking for Velocity{}
// impl Networking for Hp{}

// use bevy::ecs::query::WorldQuery;
// use std::any::Any;
// use std::sync::Arc;
// use std::collections::HashMap;

// use std::collections::HashSet;
// pub struct SnapShot{

//     resources: HashSet<Arc<dyn Reflect>>,

//     components: HashMap<Entity, Vec<Arc<dyn Reflect>>>,

//     //events: Vec<EventReader<Arc<dyn Reflect>>>,

//     //checksum: i32,
// }
// // pub struct SnapEntity{
// //     components: Vec<Arc<(dyn Reflect)>>,
// // }
// use bevy::ecs::system::Resource;

// impl SnapShot{
//     pub fn get<C: Reflect + std::fmt::Debug>(&self){
        
//         for (ent, com_vec )in self.components.iter(){
//             for com in com_vec.iter(){
//                 //let a = com.downcast_ref::<C>();
//                 println!("downcast ent{:?}: {:?}", ent, *com);
//             }
//         }
//     }
//     pub fn new() -> Self{
//         SnapShot{
//             resources: HashSet::new(),
//             components: HashMap::new(),
//         }

//     }
//     // pub fn from_query( query: dyn WorldQuery){
//     //     // .. some code here ..//
//     // }
//     pub fn with<E: Component, C: Component + Reflect + Clone>(&mut self, world: &mut World) -> &mut Self{
//         let mut query = world.query_filtered::<(Entity, &mut C), With<E>>();
//         for (ent, component) in query.iter(world){

//             // Looks crazy I know.
//             if let Some(x) = self.components.get_mut(&ent) {
//                 x.push(Arc::new(component.clone()));
//             } else {
//                 self.components.insert(ent, Vec::new());

//                 if let Some(x) = self.components.get_mut(&ent) {
//                     x.push(Arc::new(component.clone()));
//                 }
//             }
//         }
//         self
//     }   
//     pub fn with_res<C: Default + Resource + Clone + Reflect>(&mut self, world: &mut World) -> &mut Self{
//         let res = world.get_resource::<C>().unwrap();
//         //self.resources.insert(Arc::new(res.clone()));
//         self
//     }

//     // pub fn from_struct<C: SyncType>(){


//     // }
//     pub fn save(mut commands: Commands){

//     }
// }
// trait SyncType{
//     type SyncFilter;
//     fn snapshot(world: &mut World) -> SnapShot;
//     //fn snapshot2() -> SnapShot;
// }

// use std::ops::{Add, Sub};


// // impl Add for SnapShot {
// //     type Output = Self;

// //     fn add(self, other: Self) -> Self {
// //         self.resources + other.resources;
// //     }
// // }

// // impl Sub for SnapShot {
// //     type Output = Self;

// //     fn sub(self, other: Self) -> Self {
// //         Self {x: self.x - other.x, y: self.y - other.y}
// //     }
// // }

// pub struct MyType;
// impl SyncType for MyType{

//     type SyncFilter = NetSync;
    
//     fn snapshot(world: &mut World) -> SnapShot{
//         let mut snap = SnapShot::new();
//         snap
//         .with::<NetSync, Velocity>(world)
//         .with::<NetSync, Transform>(world)
//         .with::<NetSync, Id>(world)
//         .with::<NetSync, Hp>(world)
//         .with_res::<SelectedId>(world);
//         snap
//     }
//     // fn snapshot2(snap: SnapShot) -> SnapShot {
//     //     snap
//     //     .with::<Velocity>()

//     // }
// }

// fn sync_resources(snap: SnapShot){
//     // snap
//     // .res::<SelectedId>()
//     // .res::<SelectedId>()
//     // .res::<SelectedId>()
// }
// fn simple( world: &mut World ){
//     //println!("Simple Snap");
//     let snap1 = MyType::snapshot(world);

//     let snap2 = MyType::snapshot(world);

//     // let snap3 = SnapShot::shot::<NetSync>();
//     // snap3.with_res::<SelectedId>
//     //let snap3 = SnapShot::entire::<NetSync>();

//     //println!("Now lets print all components");
//     // snap1.get::<Hp>();
//     // snap1.get::<Transform>();
//     // snap1.get::<Velocity>();

//    // let diff = snap1 - snap2;
// }

// fn alt(
//     query: Query<(&Transform, &Velocity, &Hp, &Id), With<Selected>>,
//     // snaps: ResMut<reactiveSnap>,
//     res: Res<SelectedId>,
//     mut commands: Commands,
//     //snap: ResMut<SnapServer>
// ){


//      //let snap = SnapShot::from_query(query).with_resources((res));
//     // let snap = SnapShot::new();
//     // snap.with_res(res);

//     // snap.load(commands);
//     // commands.load_snap(snap); // This is better I guess
//     // for (trans, vel, hp, id) in query.iter(){
//     //     let mut snap = SnapShot::new();
//     //     snap.components.push(Arc::new(trans.clone()));
//     //     snap.components.push(Arc::new(vel.clone()));
//     //     snap.components.push(Arc::new(hp.clone()));
//     //     snap.components.push(Arc::new(id.clone()));
//     //     // for component in snap.components.iter(){
//     //     //     println!("components: {:?}", component.get());
//     //     // }
//     //     //snap.get::<Transform>();
//     // }

        
    
// }



// //use serde::{Serialize, Deserialize};

// // #[derive(Component, Reflect, Default)]
// // #[reflect(Component, Resource)]
// // struct Steps(f32);






// fn save_keys(mut commands: Commands, keys: Res<Input<KeyCode>>) {
//     if keys.just_pressed(KeyCode::S) {




//         info!("Making snapshot");
//         // This triggers saving the world the next time commands are processed.
//         // The snapshot is then sent as an event so it can be picked up by other systems.
//         commands.save::<HeySnap>();
//         //commands.save_ent::<MySnap>();
//         let with_entities: Vec<i32> = Vec::new(); // If empty, than all.
//         //let with_resources = Vec::new(); ?



//         commands.save::<HeySnap>();
//         // commands.load_without_resources::<MySnap>(save_slot.0.clone(), with_entities);

        
//         // commands.save::<MySnap>();
//         // commands.load::<MySnap>(save_slot.0.clone());
//     }
// }

// fn store_snapshot(
//     mut save_events: EventReader<SaveEvent<HeySnap>>,
//     mut entity: Query<Entity, With<Selected>>,
//     mut commands: Commands

// ) {
//     for save_event in save_events.iter() {
//         info!("Writing snapshot to save slot resource");

//         for ent in entity.iter(){
//             let com_ent = commands.entity(ent);
//             //println!("com_ent: {:?}", com_ent);
//         }

//         // Save the snapshot in a resource so we can restore it later
//         // let snap_diff = snap1 - snap2;

//         // snap.get_checksum(id);
//         // let vel = snap.get::<Velocity>(id);
//         //let encoded: Vec<u8> = bincode::serialize(&save_event.snapshot.clone()).unwrap();

//         //let des = serde_json::to_string(&save_event.snapshot.clone()).unwrap();
//         println!("{:?}", &save_event.snapshot.clone());
//     }
// }


