use bevy::{prelude::*, reflect::TypeRegistry};
use std::marker::PhantomData;

#[derive(Default)]
pub struct GoSnapPlugin<T>
where
    T: SnapType,
{
    t: PhantomData<T>,
}

impl<T: 'static + SnapType> Plugin for GoSnapPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<SnapRegistry<T>>();
        // app.init_resource::<SnapshotIdProvider<T>>();
        // app.add_event::<SaveEvent<T>>();
        app.add_system(
            save_snap::<T>
                .exclusive_system()
                .with_run_criteria(bevy::time::FixedTimestep::step(1.)),
        );
    }
}

use bevy::ecs::schedule::ShouldRun;

fn run_if_pressed(keys: Res<Input<KeyCode>>) -> ShouldRun {
    if keys.just_pressed(KeyCode::S)
    {
        return ShouldRun::Yes;
    }
    return ShouldRun::No;
}

pub fn save_snap<T: SnapType>(world: &mut World) {
    info!("Making snapshot");
    let entity = world
        .spawn()
        .insert(Transform::from_xyz(0., 1., 51.))
        .insert(Transform::from_xyz(0., 1., 1.))
        .id();
    let registry = world
        .get_resource::<SnapRegistry<T>>()
        .expect("No type registry found, did you forget to initialize the save plugin?");

    // for component_id in world.components() {
    //     //println!("id: {:?}", world.components().get_info(ComponentId::new(177)));
    //     //println!("reg: {:?}", type_registry);
    //     let reflect_component = world
    //         .components()
    //         .get_info(component_id)
    //         .and_then(|info| type_registry.get(info.type_id().unwrap()))
    //         .and_then(|registration| registration.data::<ReflectComponent>());
    // }
    use bevy::ecs::component::ComponentId;

    // let com = registry.type_registry.read()
    // .get_with_name("corgee::shared::character::MaxSpeed")
    // .and_then(
    //     |hz| hz.data::<ReflectComponent>()
    //     .and_then(|hz| hz.reflect_component(&world, entity)));

    // if !com.is_none(){
    //     println!("{:?}", com.unwrap());
    // }

    // let type_registry = registry.type_registry.read();
    // let reflect_component = world
    //     .components()
    //     .get_info(ComponentId::new(176))
    //     .and_then(|info| type_registry.get(info.type_id().unwrap()))
    //     .and_then(|registration| registration.data::<ReflectComponent>());

    //let vec = Vec::new(Transform,);

    //vec.push(world.get::<Transform>(entity).unwrap());

    // let entity = world.spawn().insert(Transform::from_xyz(0.,1.,51.)).insert(Transform::from_xyz(0.,1.,1.)).id();

    // let position = world.get::<Transform>(entity).unwrap();

    // println!("{:?}", position);
}
pub trait SnapType: 'static + Send + Sync + Default {
    fn add_types(registry: &mut TypeRegistry);
}

// #[derive(Default)]
// pub struct SaveCommand<T: SnapType> {
//     t: PhantomData<T>,
// }

// pub trait SaveCommandExt {
//     fn save<T: SnapType>(&mut self);
// }

// impl SaveCommandExt for Commands<'_, '_> {
//     fn save<T: SnapType>(&mut self) {
//         self.add(SaveCommand::<T>::default())
//     }
// }

struct SnapRegistry<T: SnapType> {
    type_registry: TypeRegistry,
    t: PhantomData<T>,
}

impl<T: SnapType> Default for SnapRegistry<T> {
    fn default() -> Self {
        let mut type_registry = TypeRegistry::default();
        T::add_types(&mut type_registry);
        Self {
            type_registry,
            t: default(),
        }
    }
}

// impl<T: SnapType> Command for SaveCommand<T> {
//     fn write(self, mut world: &mut World) {
//         let com = world.components();
//         let registry = world
//         .get_resource::<SnapRegistry<T>>()
//         .expect("No type registry found, did you forget to initialize the save plugin?");

//         println!("com: {:?}", registry.type_registry.clone());

//         //let mut query = world.query_filtered::<Entity, With<NetSync>>();

//         //
//     }
// }

/*
Api

struct MySnap;

derive(GSnap)
struct oh_no_how_to_sync_this_component;


    .add_plugin(GSnap::new());


    commands.save::<MySnap>();

*/
