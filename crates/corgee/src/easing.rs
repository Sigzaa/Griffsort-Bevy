use bevy::prelude::{Query, Plugin, App, *};
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

pub fn ease(from: &mut f32, to: f32, func: fn(f32) -> f32, time: &mut f32) -> f32{
    match *from > to{
        true =>{

        },
        false =>{

        }
    }
    //*from += step;
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

// Anime.rs

impl Plugin for Anime{
    fn build(&self, app: &mut App){
        app.add_system(anime_iterate);
    }
}

fn anime_iterate(
    mut anime_q: Query<&mut Anime>,
    time: Res<Time>
){
    for anime in &mut anime_q{
        anime.tick(time.delta_seconds());
    }
}

#[derive(Component)]
pub struct Anime(Vec<Easer>);

#[derive(Copy, Clone)]
pub struct Easer{
    val: f32,
    time: f32,
}


impl Anime{
    pub fn new(easers_amount: i8){

    }
    pub fn easer(&self, num: usize) -> Easer{
        self.0[num]
    }
    pub fn value(&self, num: usize) -> f32{
        self.0[num].val
    }
    pub fn tick(&self, timestep: f32){

    }

}
impl Easer{
    pub fn new(easers_amount: i8){

    }

    fn ease(        
        &mut self,
        easing: fn(f32) -> f32,
        to: f32,
        duration: f32,
    ){
       

    }
}

/* Api #1

.insert(Anime::new(5)))

sys () {
    anime.easer(0).ease_to(EaseOutQuat, 150., 5.);

    ...

    some_value = anime.value(0);
}


*/

/* Api #2


sys () {
    anime(&mut some_value, EaseOutQuat, 150., 5.);

    anime(&mut some_value, EaseOutQuat, 150., 5.).await;
}


*/

pub async fn anime(value: &mut f32, to: f32, easing: fn(f32) -> f32, duration: f32){

    /*
    
    val = 50;

    to = 150;

    duration = 5.;

    */

    let init_time= 14.;

    //let 

    while time(init_time, duration){


    }
}

fn time(init_time: f32, duration: f32) -> bool{
    true
}
