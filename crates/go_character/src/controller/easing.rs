use clamped::Clamp;
pub fn ease_out_quad(x: f32) -> f32 {
    return 1. - (1. - x) * (1. - x);
}
pub fn ease_out_quart(x: f32) -> f32 {
    let base = 1. - x;
    return (1. - base.powf(4.)) * x.signum();

}
pub fn ease_in_quart(x: f32) -> f32  {
    return x * x * x * x;
}

pub fn ease(from: &mut f32, to: f32, func: fn(f32) -> f32, step: f32) -> f32{
    match *from > to{
        true =>{

        },
        false =>{

        }
    }
    *from += step;
    *from = from.clamp(0., 1.);
    return func(*from);
}
pub fn val_to(val: &mut f32, to: f32, step: f32){
    match *val > to{
        true =>{
            if *val -step.abs() <= to{
                *val = to;
            } else {
                *val -= step.abs();
            }
        },
        false =>{
            if *val +step.abs() >= to{
                *val = to;
            } else {
                *val += step.abs();
            }
        }
    }
}
