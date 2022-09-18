use bevy::ecs::component::ComponentId;
use bevy::ecs::query::WorldQuery;
use bevy::reflect::ReflectMut;
use bevy::{ecs::storage::Table, prelude::*, utils::HashMap};
use std::any::TypeId;
use std::collections::HashSet;
use std::default;
use std::hash::Hash;
use std::ops::Add;
use std::rc::Rc;

pub struct AnimationDescriptor<C: WorldQuery> {
    pub to: C,
    pub duration: f32,
    pub delay: f32,
    // pub repeat: Repeat,
}

pub struct AnimEv(dyn Reflect);
// pub struct AnimationQueue<C: Component>{
//     pub queue: Vec<Animee<C>>,
// }

// impl<C: Component> Default for AnimationQueue<C>{
//     fn default() -> Self {
//         Self { queue: Vec::<Animee<C>>::new() }
//     }
// }
// impl Default for Animee{
//     fn default() -> Self {
//         Self {
//             targets: Vec::new(),// to: Box::new(Transform::from_xyz(9., 9., 9.))
//              duration: 1., delay: 0., repeat: Repeat::No}
//     }
// }
trait CanEase {
    fn ease(&self, step: f32) -> Self;
}

// #[derive(Component)]
// pub struct Anime<C: Component + Reflect>{
//     timeline: Vec<Vec<AnimationDescriptor<C>>>,

// }

#[derive(Component)]
pub struct Anim {
    //timeline: Vec<AnimDesc>
}

// impl<C: Component + Reflect> Anim <C>{
//     pub fn new() -> Self{
//         Self { timeline: Vec::<AnimDesc<C>>::new() }
//     }
//     pub fn add(&mut self, desc: AnimDesc<C>){
//         println!("adding animation");

//         self.timeline.push(desc);
//     }
//     fn update(&mut self){
//         if self.timeline.len() == 0{
//             return;
//         }
//         println!("Updating");

//         //reflect(self.timeline[0].to);
//     }
// }

pub struct AnimDesc<C: Component + Reflect> {
    pub to: C,
    pub duration: f32,
    pub delay: f32,
}

fn reflect(value: &mut dyn Reflect) {
    match value.reflect_mut()
    {
        // `Struct` is a trait automatically implemented for structs that derive Reflect. This trait
        // allows you to interact with fields via their string names or indices
        ReflectMut::Struct(value) =>
        {
            for i in 0..value.field_len()
            {
                if let Some(field) = value.field_at_mut(i)
                {
                    if field.as_any_mut().is::<f32>()
                    {
                        let cast: &mut f32 = field.downcast_mut().unwrap();

                        *cast += 4.;
                    }

                    println!("value: {:?}", field.clone_value());

                    reflect(field);
                }
            }
        }

        // `TupleStruct` is a trait automatically implemented for tuple structs that derive Reflect.
        // This trait allows you to interact with fields via their indices
        ReflectMut::TupleStruct(_) =>
        {}
        // `Tuple` is a special trait that can be manually implemented (instead of deriving
        // Reflect). This exposes "tuple" operations on your type, allowing you to interact
        // with fields via their indices. Tuple is automatically implemented for tuples of
        // arity 12 or less.
        ReflectMut::Tuple(_) =>
        {}
        // `Enum` is a trait automatically implemented for enums that derive Reflect. This trait allows you
        // to interact with the current variant and its fields (if it has any)

        // `List` is a special trait that can be manually implemented (instead of deriving Reflect).
        // This exposes "list" operations on your type, such as insertion. `List` is automatically
        // implemented for relevant core types like Vec<T>.
        ReflectMut::List(_) =>
        {}
        // `Array` is a special trait that can be manually implemented (instead of deriving Reflect).
        // This exposes "array" operations on your type, such as indexing. `Array`
        // is automatically implemented for relevant core types like [T; N].
        ReflectMut::Array(_) =>
        {}
        // `Map` is a special trait that can be manually implemented (instead of deriving Reflect).
        // This exposes "map" operations on your type, such as getting / inserting by key.
        // Map is automatically implemented for relevant core types like HashMap<K, V>
        ReflectMut::Map(_) =>
        {}
        // `Value` types do not implement any of the other traits above. They are simply a Reflect
        // implementation. Value is implemented for core types like i32, usize, f32, and
        // String.
        ReflectMut::Value(_) =>
        {}
    }
}

// // Timeline
// pub struct AnimeDescriptor<C: Component>{
//     pub targets: Vec<Entity>,
//     pub queue: AnimationQueue<C>
// }

// impl<C: Component> AnimeDescriptor<C>{
//     pub fn add(&mut self, _: Animee<C>) -> &mut Self{
//         self
//     }
// }

// impl<C: Component> Anime<C>{
//     pub fn target(&mut self, entity: Entity) -> AnimeDescriptor<C>{
//         AnimeDescriptor::<C>{
//             targets: vec![entity],
//             queue: AnimationQueue::<C>::default()
//         }
//     }
// }

// impl<C: WorldQuery> Anime <C>{
//     pub fn new() -> Self{
//         Self { timeline: Vec::<Vec::<AnimationDescriptor<C>>>::new() }
//     }
//     pub fn add(){

//     }
//     pub fn after(){

//     }

// }

// impl<C: Component + Reflect> Plugin for Anim <C>{
//     fn build(&self, app: &mut App) {
//         app
//         //.insert_resource(Anime::<C>::new())
//         .add_system(update::<C>.exclusive_system())
//         ;
//     }
// }

// impl<C: WorldQuery + Send + Sync + 'static> Plugin for Anime<C>{
//     fn build(&self, app: &mut App) {
//         app
//         //.insert_resource(Anime::<C>::new())
//         //.add_system(update::<C>.exclusive_system())
//         ;
//     }
// // }
// fn update<C: Component + Reflect>(
//     world: &mut World
// ){
//    let mut query = world.query::<(&mut Anim<C>, &mut C, Entity)>();

//     for (mut anime, mut comp, entity) in query.iter_mut(world){
//         reflect(comp.as_reflect_mut());
//     }

//     let vec = world.query::<(&mut Anim, Entity)>().iter(&world)
//     .collect::<Vec<_>>().clone();

//     let mut vec2 = Vec::<(Entity, ComponentId)>::new();

//     for (anime, entity) in vec{
//         // let type_id = anime.1.unwrap();

//         // let component_id = world.components().get_resource_id(type_id);

//         let component_info_vec = world.inspect_entity(entity);

//         for component_info in component_info_vec{

//             vec2.push((entity, component_info.id()));
//             //println!("ComponentId: {:?}", );
//         }

// //     }
//     for (entity, component_id) in vec2{
//         unsafe{
//             let target = world.get_mut_by_id(entity, component_id).unwrap().into_inner().deref_mut::<dyn Reflect>();

//             //println!("target: {:?}", target.1);
//         }

//     }

//}

// fn update(
//     world: &mut World
// ){

//     //for (mut anime, ent) in query.iter(world)

// for (entity, mut anime) in &mut anime_query{
//let target = anime.0[0].target.as_mut();
// println!("inside plugin: {}", anime.1.unwrap().translation);
// anime.1.unwrap().translation += Vec3::new(0.1, 0.2, 0.);
// target += 4.;
// if let Ok(target) = value.downcast::<String>() {
//     println!("String ({}): {}", string.len(), string);
// }

//}
//}
