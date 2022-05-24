pub use priority_queue::PriorityQueue;
use crate::shared::resources::*;
/*
    Client side:

        Each synced player has a buffer with inputs ()
        There is a resource with all snapshots.



*/
#[derive(Default)]
pub(crate) struct GoBuf{
    queue: PriorityQueue<BufContent, i32>,
    size_limit: i32,
}

impl GoBuf{
    pub fn new(size_limit: i32) -> Self {
        GoBuf{
            queue: PriorityQueue::new(),
            size_limit,
        }
    }
    pub fn get(&self, index: i32){

    }
    pub fn push(content: BufContent, tick: i32){

    }
    pub fn pop(){

    }
    pub fn pop_n_split(){

    }
    pub fn last(){

    }
    pub fn cut_after(){

    }
    pub fn cut_before(){

    }
    pub fn pop_n_cut_after(){

    }
    pub fn pop_n_cut_before(){
        
    }
    pub fn change_tick_count(){

    }
    pub fn remake(){

    }
    fn cut_excess(){

    }
}