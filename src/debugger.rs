use actions::Actions;
use bevy::{prelude::*, diagnostic::{FrameTimeDiagnosticsPlugin, Diagnostics}};
use bevy_draw_debug_tools::{DebugLines, DebugLinesExt};
use bevy_inspector_egui::{InspectorPlugin, Inspectable};
use bevy_debug_text_overlay::{screen_print, OverlayPlugin};
use bevy_rapier3d::rapier::prelude::RigidBody;
use gs_inspector::{ToScreenDebugger, ShapeDebugger};
use heroes::{Selected, CameraLink, HeadLink, Collider};
use serde::*;
use synx::Synx;

use crate::resources::Action;


pub struct GSDebugger;

impl Plugin for GSDebugger{
    fn build(&self, app: &mut App) {
        app
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_system(print_to_screen)
        .add_system(transform)
        ;
    }
}

fn print_to_screen(
        scrd: Res<ToScreenDebugger>,
        diagnostics: Res<Diagnostics>,
        actionsq: Query<&Actions<Action>, With<Selected>>,

){
    if scrd.show_fps{
        if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(fps_avg) = fps_diagnostic.average() {
                screen_print!("fps: {fps_avg:.0}");
            }
        }
    }
    if scrd.show_inputs{
        for action in &actionsq{
            screen_print!("inputs: {:?}", action.pressed);
        }

    }



}

fn show_collider(
        q: Query<(&Collider, &Transform)>,
        shape_conf: Res<ShapeDebugger>,
        lines: ResMut<DebugLines>,
        ){
    if shape_conf.rb_colliders.enable{
        for (collider, transform) in &q{
            todo!();
            /*
            lines.shape(

            );

            */
        }

    }
}

fn transform(
        scrd: Res<ToScreenDebugger>,
        q: Query<(&Transform, &CameraLink, &HeadLink), With<Selected>>,
        q2: Query<&Transform>,
        ){

    if scrd.show_transform{
        for (body_transform, cam_link, head_link) in &q{

            let head_transform = q2.get(head_link.0).unwrap();

            let cam_transform = q2.get(cam_link.0).unwrap();

            screen_print!("body transform: {:?}", body_transform);
            screen_print!("head transform: {:?}", head_transform);
            screen_print!("camera transform: {:?}", cam_transform);

        }

    }
}

