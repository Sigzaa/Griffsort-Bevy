mod resources;
mod systems;
mod plugin;
mod example;

pub use plugin::ActionsPlugin;
pub use systems::{update_inputs, collect_actions};
pub use resources::{Actions, Keybindings};

pub(crate) use resources::*;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap};

    use super::*;

    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};
    use ron::ser::{to_string_pretty, PrettyConfig};
    use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};
    use std::fs;

    #[test]
    fn it_works() {

        let mut app = App::new();
        app.add_startup_system(setup);

        
        

        // let mut bindings = Keybindings::<Action>::default();
        // bindings.keybindings.insert(Inp::Keyboard(KeyCode::Space), Action::Jump);
        // bindings.keybindings.insert(Inp::Mouse(MouseButton::Left), Action::Left);




        app.add_plugin(ActionsPlugin::<Action, Selected>::new("./bindings.ron", "./bindings.default.ron"));

        
        //app.add_system(update_actions);
        //app.add_system(collect_actions::<Selected, Action>);

        app.update();

    }

    #[derive(Component)]
    struct Selected;

    fn setup(mut commands: Commands){
        commands.spawn().insert(Actions::<Action>::new());
    }

    #[derive(Hash, PartialEq, Eq, Debug, Deserialize, Serialize, Clone)]
    enum Action{
        Left,
        Shoot,
        Back,
        Jump,
        Sprint,
        Abil1,
        Command(String),
    }

    // impl Default for Action{
    //     fn default() -> Self {
    //         Action::None
    //     }
    // }
    // fn bind(key: KeyCode) -> Action{
    //     match key{
    //         KeyCode::A => Action::Left,
    //         KeyCode::S => Action::Back,
    //         _=> Action::None
    //     }
    // }


}
