
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
    pub fn new(charges_available: i8, cd_time: f32) -> Self {
        Self {
            charges_available,
            charges_left: charges_available,
            cd_time,
            timer: 0.,
            is_locked: false,
        }
    }
}

pub trait CooldownManager {
    fn try_to_use(&mut self) -> bool{
        return if self.is_ready(){
            self.used();
            true
        }
        else {
            false
        }
    }
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