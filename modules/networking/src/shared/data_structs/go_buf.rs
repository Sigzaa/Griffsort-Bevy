use bevy::log::warn;
use std::ops::Index;
/*
    This is an easy Interface on vector.
    Very slow...
*/
#[derive(Default)]
pub(crate) struct GoBuf<T>
{
    vec: Vec<Box<T>>,
    capacity: i32,
}
#[derive(PartialEq)]
struct Box<T> {
    content: T,
    tick: i32,
}

impl<T> GoBuf<T> {
    pub fn new(capacity: i32) -> Self {
        GoBuf {
            vec: Vec::new(),
            capacity,
        }
    }
    pub fn push(&self, content: T, tick: i32) {
        self.vec.push(Box { content, tick });
        self.sort();
    }
    pub fn cut_after(&self, tick: i32) {
        let index = self.tick_to_index(tick);
        self.vec.split_off(index);
    }
    pub fn cut_before(&self, tick: i32) {
        let index = self.tick_to_index(tick);
        self.vec.drain(0..index);
    }
    pub fn change_tick_count(&self) {
        todo!();
    }
    pub fn highest_tick(&self) -> i32{
        -1
    }
    fn sort(&self) {
        self.vec.sort_by(|a, b| a.tick.cmp(&b.tick));
    }
    fn cut_excess(&self) {
        if self.capacity <= self.vec.len() as i32 {
            self.vec.remove(0);
        }
    }
    fn tick_to_index(&self, tick: i32) -> usize {
        let buf_box = self.tick_to_box(tick);
        if buf_box != None {
            let buf_box = buf_box.unwrap();
            return self.vec.iter().position(|&r| r == buf_box).unwrap();
        }
        return 0;
    }
    fn tick_to_box(&self, tick: i32) -> Option<Box<T>> {
        // -> Box
        for i in self.vec {
            if i.tick == tick {
                return Some(i);
            }
        }
        warn!("No BufBox found at tick: {}", tick);
        None
    }
}
impl<T> Index<i32> for GoBuf<T> {
    // -> BufContent
    type Output = Option<T>;
    fn index(&self, tick: i32) -> &Self::Output {
        for i in self.vec {
            if i.tick == tick {
                return &Some(i.content);
            }
        }
        warn!("No Content found at tick: {}", tick);
        &None
    }
}
