use super::plugin::Character;
use bevy::prelude::*;
use corgee::*;
pub use components::*;
pub struct SpawnChar(pub &'static str, pub i32, pub i32); // Character code, team code, id.

#[derive(Debug)]
pub struct CDProps {
    charges_available: i8,
    cd_time: f32,
    timer: f32,
    is_locked: bool,
    charges_left: i8,
}
impl CDProps {
    pub fn default() -> Self {
        Self {
            charges_available: 4,
            charges_left: 2,
            cd_time: 4.,
            timer: 4.,
            is_locked: false,
            
        }
    }
    pub fn new(ab_amount: f32) -> Self {
        todo!();
    }
}

pub trait CooldownManager {
    fn is_ready(&mut self) -> bool 
    {
        let charges_left = self.charges_left();
        let props = self.pull_props();

        if charges_left >= 1{
            return true;
        } else {
            return false;
        }
    }
    fn charges_left(&mut self) -> i8 {
        self.pull_props().charges_left.clone()
    }
    fn step(&mut self, time: f32) 
    {
        if self.pull_props().timer > 0.
        {
        self.pull_props().timer -= time;
        }
        if self.pull_props().timer <= 0.
        {
            
            self.add_charge(1);

            if self.charges_in_cd() >= 1
            {
                self.pull_props().timer = self.pull_props().cd_time;
            }
        }
    }
    fn used(&mut self) 
    {
        let is_ready = self.is_ready().clone();
        let is_locked = self.is_locked().clone();
        let props = self.pull_props();

        if is_ready && !is_locked
        {
            props.timer = props.cd_time;
            props.charges_left -= 1;

        }
    }
    fn lock(&mut self) 
    {
        self.pull_props().is_locked = true;
    }
    fn unlock(&mut self) 
    {
        self.pull_props().is_locked = false;
    }
    fn lock_for(&mut self, time: f32){
        todo!();
    }
    fn is_locked(&mut self,) -> bool
    {
        self.pull_props().is_locked
    }
    fn add_charge(&mut self, amount: i8)
    {
        if self.charges_in_cd() >= 1{
            self.pull_props().charges_left += amount;
        }
    }
    fn charges_in_cd(&mut self) -> i8
    {
        let props = self.pull_props();
        props.charges_available - props.charges_left
    }
    fn pull_props(&mut self) -> &mut CDProps;
}

/* Character Impl File */
//#[derive(Component, AbilCD)]

/* End of the character Impl File */

// impl AbilCooldown for CDEsc{

// }

pub const CHARACTER: u32 = 0b11;

#[derive(Default)]
pub struct ShowRay(pub bool);


#[derive(Default)]
pub struct Controller<T: 'static> {
    pub char_type: T,
}

mod components {
    use bevy::prelude::*;

    #[derive(Component)]
    pub struct ZHead;

    #[derive(Component)]
    pub struct SelCam;

    #[derive(Component)]
    pub struct SelHead;

    #[derive(Component)]
    pub struct Grounded;

    #[derive(Component)]
    pub struct RideHeight(pub f32);

    #[derive(Component)]
    pub struct ShootTimer(pub Timer);
    #[derive(Component)]
    pub struct IsReadyShoot(pub bool);

    #[derive(Component)]
    pub struct ETimer(pub f32);

    #[derive(Component)]
    pub struct QTimer(pub f32);

    #[derive(Component)]
    pub struct FTimer(pub f32);

    #[derive(Component)]
    pub struct ShiftTimer(pub f32);

}
