mod plugin;
mod resources;

use bevy::prelude::Vec3;
pub use plugin::*;
pub use resources::*;

use async_std::task;
use std::{ops::Add, thread, time};

#[cfg(test)]
mod tests {
    use std::{
        borrow::{Borrow, BorrowMut},
        rc::Rc,
    };

    use super::*;
    use bevy::{
        input::InputPlugin,
        prelude::*,
        reflect::{
            ReflectDeserialize, ReflectMut, ReflectRef, ReflectSerialize, StructInfo, TypeInfo,
            TypeRegistry, TypeRegistryInternal,
        },
        window::WindowPlugin,
    };

    type LeftHandType<'a> = (&'a mut Transform, &'a mut Camera);

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

                            *cast = 4.;
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

    #[test]
    fn api() {}

    fn system(mut query: Query<(&mut Anim, &mut Transform)>) {
        for (mut anim, mut transform) in &mut query
        {
            //(transform, style) = anim.update().get(vec!["crosshair to attack", "crosshair to "]);
            //println!("init query");

            let mut trans = Transform::default();
            trans.translation[0] += 16.;

            // anim.add(AnimDesc {
            //     to: *transform,
            //     duration: 3.,
            //     delay: 0.
            // });

            /*
                transform.translation[0] = anime.ease({
                    from: transform.translation[0],
                    add: 10.
                    name: "crosshair to attack"

                    ..Default::default();
                });

                Or...

                anime.add({
                    from: transform.translation[0],
                    add: 10.,
                    name: "crosshair to attack",

                    ..Default::default();
                });

                transform.translation[0] = anime.get(vec!["crosshair to attack"]);

            */

            println!("translation: {}", transform.translation);
        }
    }

    #[test]
    fn pointers_experiments() {
        let mut val = 5;

        let mut bo = Box::new(val);

        val = 3;

        // if let Ok(boval) = bo.downcast::<i32>(){

        // }
        let mut boval = *bo;

        let mut num = 1;

        let rc = Rc::new(num);

        num = 5;

        //rc = 7;
        //boval = 10;

        println!("value: {num}, pointer: {}", *rc);
    }

    #[test]
    fn it_works() {
        let q1 = Quat::from_rotation_x(10.);
        let q2 = Quat::from_rotation_y(-10.);

        let q3 = q1 * q2;

        // println!("{q1} + {q2} = {q3}");

        let mut tr = Transform::default();
        let mut tr2 = Transform::default();

        let tyinfo = tr.get_type_info();

        match tyinfo
        {
            TypeInfo::Struct(struc) =>
            {
                for i in struc.iter()
                {
                    //
                }
            }
            _ =>
            {}
        }

        // reflect(&mut tr);

        // println!("after: {:?}", tr);

        let vec1 = vec![13., 15.];

        let vec2 = vec![3., 5.];

        //let sumvec = vec1 + vec2;

        /*
            init = 5.;

            to = To::Relative(+4.);

            path = 4.;

            let mut timer;

            loop{

                // val in range 0..1
                val = func(timer);

                timer++;



            }

        */

        //println!("vec {}");

        let tr3 = tr2.translation + tr.translation;

        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
        .add_plugin(WindowPlugin::default())
        .add_plugin(InputPlugin::default())

         //.add_plugin(Anim::new())
        // .add_plugin(AnimeRes::<Config>::new())

        // new AnimeRes::<Config>;
        // AnimeRes::<Config>::new();
        //.add_plugin(Anim::<Transform>::new())
        // .add_plugin(Anime::<(LeftHand, RightHand, Camera)>::new())
        // .add_plugin(Anime::<Crosshair>::new())
        // .add_plugin(Anime::<SvgCrosshair>::new())
        .add_startup_system(setup)
        .add_system(init)
        //.add_system(mainloop)
        ;

        // app.world.spawn()
        // .insert(Transform::default())
        // .insert(Anime::new());

        // app.add_startup_system(init);

        // app.add_system(test);

        for i in 0..10
        {
            app.update();
        }
    }

    fn mainloop() {
        println!("iter");
    }
    #[derive(Component)]
    struct Custom(f32);

    fn setup(mut commands: Commands) {
        commands
            .spawn()
            .insert(Transform::default())
            // .insert(Anim::<Transform>::new())
            .insert(Custom(43.));
    }

    fn init(// animeev: EventWriter<AnimEv>,
        //mut query: Query<(&mut Anim<Transform>, &mut Transform)>,
    ) {
        //println!("init system");

        // for (mut anim, mut transform) in &mut query{

        //     (transform, style) = anim.sync();
        //     //println!("init query");

        //     let mut trans = Transform::default();
        //     trans.translation[0] += 16.;

        //     anim.add(AnimDesc {
        //         to: *transform,
        //         duration: 3.,
        //         delay: 0.
        //     });

        //     println!("translation: {}", transform.translation);

        // }
    }

    struct MyComp {
        value1: f32,
        value2: bool,
    }

    fn test(// mut query: Query<&mut Anime>,
        //mut resource_anim: EventWriter<AnimeResEv<SomeResource>>,
    ) {
    }

    #[macro_export]
    macro_rules! anime {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<f32> = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
}
