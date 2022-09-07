mod resources;
mod systems;
mod plugin;
mod example;

pub use plugin::ActionsPlugin;
pub use systems::{update_inputs, collect_actions};
pub use resources::{Actions, Keybindings};


#[cfg(test)]
mod tests {

    use super::*;

    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

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


}
