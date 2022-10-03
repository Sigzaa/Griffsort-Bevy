use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

pub trait DebugLinesExt {
    fn rectangle_colored(&self);
    fn cylinder_colored(&self, start: Vec3, end: Vec3, duration: f32, color: Color);
    fn sphere_colored();
    fn ray_cast();
}

impl DebugLinesExt for DebugLines {
    fn rectangle_colored(&self) {
        todo!()
    }
    fn cylinder_colored(&self, _start: Vec3, _end: Vec3, _duration: f32, _color: Color) {
        todo!()
    }
    fn sphere_colored() {
        todo!()
    }

    fn ray_cast() {
        todo!()
    }
}
