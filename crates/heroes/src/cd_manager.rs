#[derive(Debug)]
pub struct CDProps {
    pub charges: i8,
    timers: [f32; 5],
}
impl CDProps {
    pub fn default() -> Self {
        Self {
            charges: 1,
            timers: [0.; 5],
        }
    }
    pub fn new(charges: i8) -> Self {
        Self {
            charges,
            timers: [0.; 5],
        }
    }
}

pub trait CooldownManager {
    fn is_ready(&mut self, timer_index: usize) -> bool {
        let props = self.pull_props();

        if props.charges > 0 && !self.is_cooldown(timer_index)
        {
            return true;
        }
        else
        {
            return false;
        }
    }
    fn left(&mut self) -> i8 {
        self.pull_props().charges
    }
    fn is_cooldown(&mut self, timer_index: usize) -> bool {
        if self.pull_props().timers[timer_index] > 0.
        {
            return true;
        }
        false
    }
    fn is_empty(&mut self) -> bool {
        if self.pull_props().charges == 0
        {
            return true;
        }
        false
    }
    fn tick_timers(&mut self, time: f32) {
        let mut props = self.pull_props();

        for i in 0..props.timers.len()
        {
            if props.timers[i] >= 0.
            {
                props.timers[i] -= time;
            }
        }
    }
    fn add(&mut self, amount: i8) {
        self.pull_props().charges += amount;
    }
    fn full(&mut self, amount: i8) {
        self.pull_props().charges = amount;
    }
    fn cooldown(&mut self, timer_index: usize, time: f32) {
        let mut props = self.pull_props();
        props.timers[timer_index] = time;
    }
    fn pull_props(&mut self) -> &mut CDProps;
}
