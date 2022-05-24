pub use priority_queue::PriorityQueue;
use crate::shared::resources::*;
use std::ops::Index;
/*
    This is an Interface on vector.
    Very slow...
*/

pub(crate) struct GoBuf{
    vec: Vec<Box>,
    capacity: i32,
}
#[derive(PartialEq)]
struct Box{
    content: BufContent,
    tick: i32,
}

impl GoBuf{
    pub fn new(capacity: i32) -> Self {
        GoBuf{
            vec: Vec::new(),
            capacity,
        }
    }
    pub fn push(&self, content: BufContent, tick: i32){
        self.vec.push(Box{content, tick});
    }
    pub fn cut_after(&self, tick: i32){
        let index = self.tick_to_index(tick);
        self.vec.split_off(index);
    }
    pub fn cut_before(&self, tick: i32){
        let index = self.tick_to_index(tick);
        self.vec.drain(0..index);
    }
    pub fn change_tick_count(){

    }
    fn cut_excess(&self){
        if self.capacity <= self.vec.len() as i32{
            self.vec.remove(0);
        }
    }
    fn tick_to_index(&self, tick: i32) -> usize{
        self.vec.iter().position(|&r| r == self.index_box(tick)).unwrap()
    }
    fn index_box(&self, tick: i32) -> Box{ // -> Box
        for i in self.vec{
            if i.tick == tick{
                return i;
            }
        }
        panic!("index not found");
    }
}
impl Index<i32> for GoBuf { // -> BufContent
    type Output = BufContent;
    fn index(&self, tick: i32) -> &Self::Output {
        for i in self.vec{
            if i.tick == tick{
                return &i.content;
            }
        }
        panic!("index not found");
    }
}
