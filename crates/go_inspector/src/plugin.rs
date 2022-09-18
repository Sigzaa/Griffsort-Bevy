// use super::resources::*;
// use super::widgets::*;
// use bevy::{ app::AppExit,
//     diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
//     prelude::*,
// };
// use bevy_egui::{egui, EguiContext, EguiPlugin};
// use egui::Ui;
// use std::process::Command;

// #[derive(Default)]
// struct GVersion(&'static str);

// pub struct Inspector {
//     pub game_version: &'static str,
// }
// impl Plugin for Inspector {
//     fn build(&self, app: &mut App) {
//         app.add_plugin(FrameTimeDiagnosticsPlugin::default())
//             .insert_resource(Update::default())
//             .insert_resource(OpenTab::Console)
//             .insert_resource(GVersion(self.game_version))
//             .insert_resource(Stats::new())
//             .insert_resource(Config::new())
//             .insert_resource(Console::new())
//             .insert_resource(gs_inspectorToggle::default())
//             .add_plugin(EguiPlugin)
//             .add_system(show_gs_inspector)
//             .add_system(gs_inspector);
//     }
// }

// fn gs_inspector(
//     mut egui_context: ResMut<EguiContext>,
//     mut insp: ResMut<gs_inspectorToggle>,
//     diagnostics: Res<Diagnostics>,
//     mut tab: ResMut<OpenTab>,
//     state: Res<State<GameState>>,
//     mut stats: ResMut<Stats>,
//     mut config: ResMut<Config>,
//     mut cons: ResMut<Console>,
//     keys: Res<Input<KeyCode>>,
//     version: Res<GVersion>,
//     mut update: ResMut<Update>,
//     mut reload_req: EventWriter<ReloadRequest>,
//     mut windows: ResMut<Windows>,
//     mut app_exit_events: EventWriter<AppExit>
// ) {
//     // egui_context.ctx_mut().set_visuals(egui::Visuals {
//     //     dark_mode: false,
//     //     ..Default::default()
//     // });
//     if insp.0 {
//         egui::Window::new("gs_inspector")
//             //.fixed_size([150.0, 340.0])
//             .default_pos([200., 200.])
//             .collapsible(false)
//             .resizable(false)
//             .show(egui_context.ctx_mut(), |ui| {
//                 ui.horizontal(|ui| {
//                     ui.selectable_value(&mut *tab, OpenTab::Console, "Console");
//                     ui.selectable_value(&mut *tab, OpenTab::Debug, "Debug");
//                     //ui.selectable_value(&mut *tab, OpenTab::Heroes, "Heroes");
//                     ui.selectable_value(&mut *tab, OpenTab::Config, "Config");
//                     ui.selectable_value(&mut *tab, OpenTab::About, "About");
//                     egui::widgets::global_dark_light_mode_switch(ui);
//                 });
//                 match &*tab {
//                     OpenTab::Console => {
//                         let response = ui.add(egui::TextEdit::singleline(&mut cons.string));
//                         if response.changed() {
//                             // …
//                         }
//                         if response.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
//                             // …

//                             if cons.string == String::from("state") {
//                                 println!("Current state is: {:?}", state.current());
//                             }
//                             let str_buf = cons.string.clone();
//                             cons.history.push(str_buf);
//                             cons.iter = cons.history.len();
//                             cons.string = String::from("");
//                         }
//                         if ui.input().key_pressed(egui::Key::ArrowUp) && cons.iter != 0 {
//                             // …
//                             cons.iter -= 1;
//                             let in_his = cons.history[cons.iter].clone();
//                             cons.string = in_his;
//                         }
//                         if ui.input().key_pressed(egui::Key::ArrowDown)
//                             && cons.history.len() > cons.iter
//                         {
//                             // …

//                             cons.iter += 1;
//                             if cons.history.len() == cons.iter {
//                                 cons.string = String::from("");
//                             } else {
//                                 let in_his = cons.history[cons.iter].clone();
//                                 cons.string = in_his;
//                             }
//                         }
//                     }
//                     OpenTab::Debug => {
//                         egui::Grid::new("my_grid")
//                             .num_columns(2)
//                             .spacing([130.0, 9.0])
//                             .striped(true)
//                             .show(ui, |ui| {
//                                 ui.label("General:");
//                                 ui.end_row();

//                                 ui.label("Show FPS");
//                                 ui.add(toggle(&mut stats.fps));
//                                 ui.end_row();

//                                 ui.end_row();
//                                 ui.label("Networking:");
//                                 ui.end_row();

//                                 ui.label("Show Tickrate");
//                                 ui.add(toggle(&mut stats.tick));
//                                 ui.end_row();

//                                 ui.label("Show Buffer Plot");
//                                 ui.add(toggle(&mut stats.net_buffer));
//                                 ui.end_row();

//                                 // ui.label("Open Networking debug");
//                                 // ui.add(toggle(&mut stats.fps));
//                                 // ui.end_row();

//                                 ui.end_row();
//                                 ui.label("State:");
//                                 ui.end_row();

//                                 ui.label("Show current state");
//                                 ui.add(toggle(&mut stats.state));
//                                 ui.end_row();

//                                 ui.label("Show state list");
//                                 ui.add(toggle(&mut stats.state_list));
//                                 ui.end_row();
//                             });
//                     }
//                     OpenTab::About => {
//                         ui.separator();
//                         ui.label(format!("Griffsort v{}", version.0));
//                         if ui
//                             .add_enabled(!update.is_update, egui::Button::new("Update"))
//                             .clicked()
//                         {
//                             update.is_update = true;
//                             update_game(&mut update);
//                         }
//                         if ui
//                             .add_enabled(!update.is_update, egui::Button::new("Downgrade"))
//                             .clicked()
//                         {
//                             update.is_update = true;
//                             downgrade();
//                         }
//                         if update.is_update {
//                             ui.add(egui::ProgressBar::new(update.progress).animate(true));
//                         }
//                     }
//                     OpenTab::Config => {
//                         egui::Grid::new("my_grid")
//                             .num_columns(2)
//                             .spacing([130.0, 9.0])
//                             .striped(true)
//                             .show(ui, |ui| {
//                                 ui.label("General:");
//                                 ui.end_row();
//                                 if ui.button("reload config").clicked() {
//                                     reload_req.send(ReloadRequest);
//                                 }
//                                 ui.end_row();
//                                 ui.label("Close with Del:");
//                                 ui.add(toggle(&mut config.exit_on_del));
//                                 ui.end_row();
//                             });
//                     }
//                     _ => {}
//                 }
//             });
//     }
//     if config.exit_on_del && keys.pressed(KeyCode::Delete) {
//         info!("Exiting...");
//         app_exit_events.send(AppExit);
//     }
//     if stats.fps {
//         egui::Window::new("fps")
//             .resizable(false)
//             .title_bar(false)
//             .anchor(egui::Align2::LEFT_TOP, [10., 10.])
//             //.min_width(500.)
//             .show(egui_context.ctx_mut(), |ui| {
//                 if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
//                     if let Some(average) = fps.average() {
//                         // Update the value of the second section
//                         ui.label(format!("fps: {:.0}", average));
//                     }
//                 }
//             });
//     }
//     if stats.state {
//         egui::Window::new("State")
//             .resizable(false)
//             .title_bar(false)
//             .anchor(egui::Align2::LEFT_TOP, [70., 10.])
//             //.min_width(500.)
//             .show(egui_context.ctx_mut(), |ui| {
//                 ui.label(format!("Current state: {:?}", state.current()));
//             });
//     }
//     if stats.state_list {
//         egui::Window::new("State_List")
//             .resizable(false)
//             .title_bar(false)
//             .anchor(egui::Align2::LEFT_TOP, [10., 40.])
//             //.min_width(500.)
//             .show(egui_context.ctx_mut(), |ui| {
//                 ui.label(format!("Inactive states: {:#?}", state.inactives()));
//             });
//     }
// }
// fn show_gs_inspector(
//     mut gs_inspector_toggle: ResMut<gs_inspectorToggle>,
//     keys: Res<Input<KeyCode>>,
//     mut state: ResMut<State<GameState>>,
// ) {
//     if keys.just_pressed(KeyCode::Slash) || keys.just_pressed(KeyCode::Grave) {
//         if !gs_inspector_toggle.0 {
//             state.push(GameState::Inspector);
//             gs_inspector_toggle.0 = true;
//         } else if state.current() == &GameState::Inspector {
//             state.pop();
//             gs_inspector_toggle.0 = false;
//         }
//     }
// }

// fn update_game(mut update: &mut Update) {
//     let output = if cfg!(target_os = "windows") {
//         Command::new("cmd")
//             .args(["/C", "update.bat"])
//             .output()
//             .expect("failed to execute process")
//     } else {
//         Command::new("sh")
//             .arg("-c")
//             .arg("kitty scripts/update.sh")
//             .output()
//             .expect("failed to execute process")
//     };
//     println!("out: {output:?}");
//     update.progress = 0.;
//     update.is_update = false;
// }

// fn downgrade() {
//     if cfg!(target_os = "windows") {
//         Command::new("cmd")
//             .args(["/C", "echo hello"])
//             .output()
//             .expect("failed to execute process")
//     } else {
//         Command::new("sh")
//             .arg("-c")
//             .arg("kitty scripts/downgrade.sh")
//             .output()
//             .expect("failed to execute process")
//     };
//     std::process::exit(0);
// }
