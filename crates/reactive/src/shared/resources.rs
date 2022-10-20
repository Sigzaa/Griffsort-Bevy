use serde::{Serialize, Deserialize};
use bevy::{prelude::*, reflect::TypeRegistry};
use super::data_structs::go_history::*;
use go_snap::plugin::SnapType;

pub const PhysNet: &str = "net_stage_label"; 

pub(crate) const CONST_TICKRATE: f64 = 5.;
pub(crate) const BUFFER_CAPACITY: usize = 200;
pub(crate) const INPUTS_BUFFER_CAPACITY: usize = 3;


pub struct MySnap;

#[derive(Default)]
pub struct SnapServer{
    pub types: SyncedTypes,
}
// pub trait SaveCommandExt {
//     fn shot<T: SnapType>(&mut self) -> T;
// }

impl SnapServer{
    pub fn shot<T: Component + std::fmt::Debug>(&self, world: &World, entity: Entity){
        println!("oh, hi!");
        use bevy::ecs::prelude::ReflectComponent;
        type Synced = Transform;

        let com = world.get::<T>(entity).unwrap();
        //println!("com: {:?}",com);
        //ReflectComponent::from_type().reflect_component(&world, entity);
    }
    pub fn add(){

    }
    pub fn new(){

    }
    // pub fn altshot<T: Component>(query: Query) -> Vec<T>{

    // }
}

pub struct SnapShot2<T: Default + Component>{
        entities: Vec<SnapEntity<T>>,
        resources: Vec<T>
}
pub struct SnapEntity<T: Component>{
    components: Vec<T>, 
    entity: Entity,
}
impl<T: Default + Component> SnapShot2 <T>{
    // pub fn new() -> SnapShot2<>{

    // }
}

use bevy::ecs::query::QueryIter;
use bevy::ecs::query::WorldQuery;


pub fn snapshot<T>(){

}

// impl SaveCommandExt for Commands<'_, '_> {
//     fn shot<T: SnapType>(&mut self) -> T{
//         //self.add(SaveCommand::<T>::default())
//     }


// }



#[derive(Default)]
pub struct SyncedTypes{

}

// impl SyncedTypes{
//     pub fn add<T>(&self, q: Query<T>) -> &Self{

//         &self
//     }
//     pub fn remove<T>(){

//     }
// }



// Snapshots -->
#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct SnapShot;

// impl SnapType for SnapShot {
//     fn add_types(registry: &mut TypeRegistry) {
//         registry.write().register::<Transform>();
//         registry.write().register::<GoInputs>();
//         registry.write().register::<GoRot>();
//     }
// }

#[derive(Component, Reflect, Default)]
// And be marked as components
#[reflect(Component)]
struct Player(f32);

// pub fn add_id_provider(
//     q: Query<Entity, Added<NetSync>>,
//     mut snapshot_id_provider: ResMut<SnapshotIdProvider<SnapShot>>,
//     mut commands: Commands,
// ){
//     for ent in q.iter(){
//         commands.entity(ent).insert(snapshot_id_provider.next()).insert(Player(3.));
//     }
// }

#[derive(Default)]
pub struct RollbackType{
    type_registry: Vec<TypeRegistry>,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum GenericMessages {
    PlayerConnected { id: u64 },
    PlayerDisconnected { id: u64 },
    ClientInputs {
        id: i32,
        tick: i32,
        inputs: [Inputs; INPUTS_BUFFER_CAPACITY],
    },
    World{
        tick: i32,
        //inputs: Vec<Inputs>, // TODO: remove it and handle with snapshots.
        snap: SnapShot,
    },
    Chat{
        tick: i32,
        //msg: &'static str,
    },
    ChooseCharacter{
        code: f32
    }
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct Inputs{
    pub ginp: f32,
    pub gorot: f32,
}
#[derive(Default)]
pub(crate) struct SnapBuffer(pub History<SnapShot>);

#[derive(Component, Clone)]
pub(crate) struct InputsBuffer(pub History<Inputs>); // Collecting all 
// <--

#[derive(Default)]
pub struct TickCount(pub i32);

#[derive(Default)]
pub(crate) struct TickRate(pub bevy::prelude::Timer);

#[derive(Component)]
pub struct NetSync;
