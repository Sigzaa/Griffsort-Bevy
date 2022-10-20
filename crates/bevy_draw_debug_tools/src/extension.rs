use bevy::{prelude::*, render::mesh::Indices};
use bevy_egui::{egui, EguiContext, EguiPlugin};
pub use bevy_prototype_debug_lines::*;
use bevy_rapier3d::prelude::Collider;

use crate::{MeshData, RayIntersectionExt};

pub trait DebugLinesExt {
    fn ray_cast_with_normal(&mut self, start: Vec3, direction: Vec3, max_toi: f32, intersaction: Option<(Entity, impl RayIntersectionExt)>, duration: f32, color: Color);
    fn ray_cast(&mut self, start: Vec3, end: Vec3, toi: f32, duration: f32, color: Color);
    fn distance(&mut self, start: Vec3, end: Vec3, duration: f32, color: Color, ctx: &mut EguiContext);
    fn point(&mut self, position: Vec3, duration: f32, color: Color);
    fn shape(&mut self, position: Vec3, rotation: Quat, shape: &impl MeshData,duration: f32, color: Color);
    fn arrow(&mut self, start: Vec3, end: Vec3, duration: f32, color: Color);
}

impl DebugLinesExt for DebugLines {

    fn ray_cast_with_normal(&mut self, start: Vec3, direction: Vec3, max_toi: f32, intersaction: Option<(Entity, impl RayIntersectionExt)>, duration: f32, color: Color) {
        self.line_colored(start, start + direction * max_toi, duration, color);
        if let Some((entity, intercestion)) = intersaction{

            let prop = intercestion.get_ray_intersection();

            self.line_colored(start, prop.point, duration, Color::BLACK);
            self.line_colored(prop.point, prop.point + prop.normal * 3., duration, Color::BLACK);
            self.point(prop.point, duration, color);
        }
    }
    fn ray_cast(&mut self, start: Vec3, end: Vec3, toi: f32, duration: f32, color: Color) {
        todo!()
    }

    fn distance(&mut self, start: Vec3, end: Vec3, duration: f32, color: Color, ctx: &mut EguiContext) {
        // Future feature

        todo!()
//        self.line_colored(start, end, duration, color);
//
//        let middle_in_3d = (start + end) /2.;
//
//        egui::Window::new(format!("distance from {start} to {end}"))
//        .current_pos()
//        .show(ctx.ctx_mut(), |ui| {
//            ui.label("world");
//        });
    }
    fn point(&mut self, position: Vec3, duration: f32, color: Color) {
        self.line_colored(position, position + Vec3::new(0., 0.5, 0.), duration, color);
        self.line_colored(position, position + Vec3::new(0.5, 0., 0.), duration, color);
        self.line_colored(position, position + Vec3::new(0., 0., 0.5), duration, color);
    }

    fn shape(&mut self, position: Vec3, rotation: Quat, shape: &impl MeshData, duration: f32, color: Color) {

        let vertices = shape.get_ddt_vertices().unwrap();
        let mut prev = position;

        for index in shape.get_ddt_indices().unwrap().iter(){

            let new = rotation.mul_vec3(Vec3::from_array(vertices[*index as usize]))  + position;

            self.line_colored(prev, new, duration, color);

            prev = new;
        }
    }

    fn arrow(&mut self, start: Vec3, end: Vec3, duration: f32, color: Color) {
        self.line_colored(start, end, duration, color);

        let shape = Collider::cone(0.4, 0.2);


        //self.shape(end, &shape, duration, color);
        todo!()
    }
}

