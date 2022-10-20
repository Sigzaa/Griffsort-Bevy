use functions::*;
pub enum Repeat {
    Times(i32),
    Infinity,
    No,
}

pub enum To {
    // Relative(impl Hash),
    // Absolute(impl Hash),
    RotateX(),
    RotateY(),
    RotateZ(),
}

pub enum Ease {
    In,
    Custom(fn(f32) -> f32),
}

pub fn get_function(ease: Ease) -> fn(f32) -> f32 {
    return match ease
    {
        Ease::In => todo!(),
        Ease::Custom(func) => func,
    };
}

mod functions {}
