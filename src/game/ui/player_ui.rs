use bevy::{
    prelude::*,
};
use crate::game::components::{ filters::*};
pub struct PlayerUi;
impl Plugin for PlayerUi {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(crosshair)
            .add_system(crosshair_center)
            //.add_system(world::text_update_system)
            
            //.add_plugin(Console)
            ;
    }
}
fn crosshair_center(
    mut text: Query<&mut Style, With<Crosshair>>,
    mut windows: ResMut<Windows>
    
){

    for mut style in text.iter_mut(){
        let window = windows.get_primary_mut().unwrap();
        style.position.left = Val::Px(window.width()/2. - 0.);
        style.position.bottom = Val::Px(window.height()/2. - 8.);
    }
}
fn crosshair(
    mut commands: Commands, asset_server: Res<AssetServer>
){
    
    commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    ".",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::SALMON,
                    },
                    // Note: You can use `Default::default()` in place of the `TextAlignment`
                    TextAlignment {
                        ..Default::default()
                    },
                ),
                ..Default::default()
            })
            .insert(Crosshair);
}