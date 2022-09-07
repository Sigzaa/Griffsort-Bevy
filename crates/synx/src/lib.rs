mod plugin;

use plugin::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        /*
        
        app
        .add_plugin(Synx::<Component>::new())
        .add_plugin(Synx::<Resource>::new())

        struct SyncedComponent{

            

            #[color_picker]
            color: Vec3,
            #[separator]

            boolean: bool,
            
            #[link="site, sigzaa.com"]
            string: &'static str

            #[SelLabel] // Combobox by default
            enum: Option<f32> ,

            #[slider="-1..1"]
            integer: i32,

            #[normilized_slider]
            integer: f32,

        }


        */
    }
}
