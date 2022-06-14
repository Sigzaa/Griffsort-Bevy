use bevy::log::warn;
use std::ops::Index;
pub use std::cmp::Ord;
use std::io::*;
/*
    This is an easy Interface on vector.
    Very slow...
*/
#[derive(Default, Clone)]
pub(crate) struct History<T: Clone> {
    pub vec: Vec<Box<T>>,
    pub capacity: i32,
}
#[derive(PartialEq, Clone, Ord, Eq, PartialOrd)]
pub(crate) struct Box<T> {
    content: T,
    tick: i32,
}

impl<T> History<T>
where
    T: PartialEq + Copy,
{
    pub fn new(capacity: i32) -> Self {
        History {
            vec: Vec::new(),
            capacity,
        }
    }
    pub fn get(&self, tick: i32) -> Result<T>{ // Safe
        return match self.tick_to_index(tick){
            Ok(index) => {  
                Ok(self.vec[index].content)
            },
            Err(err) => {
                Err(err)
            }
        }
    }
    pub fn insert(&mut self, tick: i32, content: T) {
        match self.tick_to_index(tick){
            Ok(index) => {
                self.vec.insert(index, Box { content, tick });
            }
            _ => {
                self.vec.push(Box { content, tick });
            }
        }
        self.sort();
    }
    pub fn cut_after(mut self, tick: i32){
        match &self.clone().tick_to_index(tick){
            Ok(index) => {
                self.vec.split_off(*index);
            },
            Err(err) => {
                warn!("{}", err);
            }
        }
    }
    pub fn cut_before(mut self, tick: i32) {
        match &self.clone().tick_to_index(tick){
            Ok(index) => {
                self.vec.drain(0..*index);
            },
            Err(err) => {
                warn!("{}", err);
            }
        }
        
    }
    pub fn change_tick_count(&self) {
        todo!();
    }
    pub fn last_tick(&self) -> Option<i32> {
        match self.vec.last(){
            Some(last) => Some(last.tick),
            None => None,
        }
    }
    pub fn last_box() -> Box<T>{
        todo!()
    }
    pub fn last_content() -> T{
        todo!()
    }
    fn sort(&mut self) {
        self.vec.sort_by(|a, b| a.tick.cmp(&b.tick));
    }
    fn cut_excess(&mut self) {
        if self.capacity <= self.vec.len() as i32 {
            self.vec.remove(0);
        }
    }
    fn tick_to_index(&self, tick: i32) -> Result<usize> {
        for (pos, e) in self.vec.iter().enumerate() {
            if pos == tick as usize{
                return Ok(pos);
            }
        }
        Err(Error::new(ErrorKind::Other, "given element does not exist"))
    }
}
impl<T: Clone + PartialEq + Copy> Index<i32> for History<T> { // If ur shure, what element exist, use it.
    type Output = T;
    fn index(&self, tick: i32) -> &Self::Output {
        return match self.tick_to_index(tick){
            Ok(index) => {  
                &self.vec[index].content
            },
            Err(err) => {
                panic!("element not found")
            }
        }
    }
}
