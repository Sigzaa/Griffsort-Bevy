use bevy::{
    prelude::*,
};
pub mod stats;
pub mod player_ui;
//mod billboarding;
pub struct Ui;
impl Plugin for Ui {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(stats::Stats)
            .add_plugin(player_ui::PlayerUi)
            //.add_system(billboarding::name_and_hp_tags_update_system)
            //.add_system(world::text_update_system)
            
            //.add_plugin(Console)
            ;
    }
}
