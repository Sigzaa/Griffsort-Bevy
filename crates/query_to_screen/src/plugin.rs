use bevy::{prelude::*, ecs::query::WorldQuery};
use pretty_type_name::pretty_type_name;
pub struct QueryToScreenPlugin;

impl Plugin for QueryToScreenPlugin{
    fn build(&self, app: &mut App) {
        app.add_system(reflect_components.exclusive_system());
    }
}

fn reflect_components(world: &mut World){
    let arches = world.components();

    for arch in arches.iter(){
        //println!("arch: {:?}", arch);
    }
    //let mut query_state = world.query::<(&Transform, &MarkS)>();
    // for transform in query_state.iter(world){
    //     println!("transform: {:?}", transform)

    // }
    

}

pub fn query_to_screen<Q: 'static>(world: &mut World)
where
    Q: WorldQuery,
{
    let mut query_state = world.query::<Q>();


        for (i, mut value) in query_state.iter(world).enumerate() 
        
        {
            let name = pretty_type_name::pretty_type_name::<Q>();
            //println!("name{:?}", value);
                
        }

    
}

pub fn filtered_query_to_screen<Q: 'static, F: 'static>(world: &mut World){

}