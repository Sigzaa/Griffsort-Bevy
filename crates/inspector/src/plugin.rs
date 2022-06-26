use super::resources::*;
use super::widgets::*;
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use corgee::GameState;
use egui::Ui;

pub struct Inspector;
impl Plugin for Inspector {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .insert_resource(OpenTab::Console)
            .insert_resource(Stats::new())
            .insert_resource(Console::new())
            .insert_resource(InspectorToggle::default())
            .add_plugin(EguiPlugin)
            .add_system(show_inspector)
            .add_system(inspector);
    }
}

fn inspector(
    mut egui_context: ResMut<EguiContext>,
    mut insp: ResMut<InspectorToggle>,
    diagnostics: Res<Diagnostics>,
    mut tab: ResMut<OpenTab>,
    mut stats: ResMut<Stats>,
    state: Res<State<GameState>>,
    mut cons: ResMut<Console>,
) {
    egui_context.ctx_mut().set_visuals(egui::Visuals {
        dark_mode: false,
        ..Default::default()
    });
    if insp.0 {
        egui::Window::new("Inspector")
            //.fixed_size([150.0, 340.0])
            .default_pos([200., 200.])
            .collapsible(false)
            
            .show(egui_context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut *tab, OpenTab::Console, "Console");
                    ui.selectable_value(&mut *tab, OpenTab::Debug, "Debug");
                    ui.selectable_value(&mut *tab, OpenTab::Heroes, "Heroes");
                    ui.selectable_value(&mut *tab, OpenTab::About, "About");
                });
                match &*tab {
                    OpenTab::Console => {
                        let response = ui.add(egui::TextEdit::singleline(&mut cons.string));
                        if response.changed() {
                            // …
                        }
                        if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                            // …
                            
                            if cons.string == String::from("state"){
                                println!("Current state is: {:?}", state.current());
                            }
                            let str_buf = cons.string.clone();
                            cons.history.push(str_buf);
                            cons.iter = cons.history.len();
                            cons.string = String::from("");
                        }
                        if ui.input().key_pressed(egui::Key::ArrowUp) && cons.iter != 0 {
                            // …
                            cons.iter -= 1;
                            let in_his = cons.history[cons.iter].clone();
                            cons.string = in_his;
                        }
                        if ui.input().key_pressed(egui::Key::ArrowDown)
                            && cons.history.len() > cons.iter
                        {
                            // …

                            cons.iter += 1;
                            if cons.history.len() == cons.iter {
                                cons.string = String::from("");
                            } else {
                                let in_his = cons.history[cons.iter].clone();
                                cons.string = in_his;
                            }
                        }
                    }
                    OpenTab::Debug => {
                        ui.separator();
                        ui.label("General:");
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Show FPS");
                            ui.add(toggle(&mut stats.fps));
                        });
                        ui.separator();
                        ui.label("Networking:");
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Show Tickrate");
                            ui.add(toggle(&mut stats.fps));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Show Buffer Plot");
                            ui.add(toggle(&mut stats.fps));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Open Networking debug");
                            ui.add(toggle(&mut stats.fps));
                        });
                        ui.separator();
                        ui.label("State:");
                        ui.separator();
                        ui.horizontal(|ui| {
                            ui.label("Show current state");
                            ui.add(toggle(&mut stats.state));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Show state list");
                            ui.add(toggle(&mut stats.state_list));
                        });
                        //ui.checkbox(, "Show fps");
                    }
                    OpenTab::About => {
                        ui.separator();
                        ui.label("Griffsort v0.0.6");
                        ui.end_row();
                        ui.label("We should think about a license");
                        ui.end_row();
                        ui.label("Sigzaa Studio");
                    }
                    _ => {}
                }
            });
    }
    if stats.fps {
        egui::Window::new("fps")
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, [10., 10.])
            //.min_width(500.)
            .show(egui_context.ctx_mut(), |ui| {
                if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                    if let Some(average) = fps.average() {
                        // Update the value of the second section
                        ui.label(format!("fps: {:.0}", average));
                        
                    }
                }
            });
    }
    if stats.state {
        egui::Window::new("State")
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, [70., 10.])
            //.min_width(500.)
            .show(egui_context.ctx_mut(), |ui| {
                ui.label(format!("Current state: {:?}", state.current()));
                
            });
    }
}
fn show_inspector(
    mut inspector_toggle: ResMut<InspectorToggle>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State<GameState>>,
) {
    if keys.just_pressed(KeyCode::Slash) || keys.just_pressed(KeyCode::Grave) {
        inspector_toggle.0 = !inspector_toggle.0;
    }
    if keys.just_pressed(KeyCode::Escape) {
        inspector_toggle.0 = false;
    }
    if inspector_toggle.0 {
        state.push(GameState::Inspector);
    } else {
        state.pop();
    }
}
