pub fn easeOutQuad(x: f32) -> f32 {
    return 1. - (1. - x) * (1. - x);
}
pub fn easeOutQuart(x: f32) -> f32 {
    let base = 1. - x;
    return (1. - base.powf(4.)) * x.signum();

}
pub fn easeInQuart(x: f32) -> f32  {
    return x * x * x * x;
}