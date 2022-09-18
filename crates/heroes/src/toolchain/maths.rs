use bevy::prelude::*;

pub fn vec_to_speed(vec: Vec3) -> f32 {
    let a = vec[0];
    let b = vec[1];
    let c = vec[2];
    (a.powf(2.) + b.powf(2.) + c.powf(2.)).sqrt()
}
pub fn horizontal_speed(vec: Vec3) -> f32 {
    let a = vec[0];
    let b = vec[2];
    (a.powf(2.) + b.powf(2.)).sqrt()
}
