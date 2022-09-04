mod resources;
mod systems;
mod plugin;
mod example;

pub use plugin::ActionsPlugin;
pub use systems::{update_inputs, collect_actions};
pub use resources::Actions;

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

        app.add_plugin(ActionsPlugin::<Action, Selected>::new("./bindings.ron", "./bindings.default.ron"));

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
