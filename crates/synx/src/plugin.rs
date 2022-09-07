use std::marker::PhantomData;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

impl<T: Component + Sync + Send + 'static> Plugin for Synx <T>{
    fn build(&self, app: &mut App) {
        app
        .add_plugin(EguiPlugin)
        .add_system(window::<T>);
    }
}

fn window<C: Component>(
    mut egui_context: ResMut<EguiContext>,
    q: Query<&C>,
){

    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        for comp in &q{
            
        }
        ui.label("world");
    });
}
pub struct Synx<T>(PhantomData<T>);
