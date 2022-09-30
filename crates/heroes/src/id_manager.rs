use bevy::{utils::HashSet};

use crate::Id;

pub struct IdManager {
    set: HashSet<i32>,
}

impl IdManager {
    pub fn new() -> Self {
        Self {
            set: HashSet::new(),
        }
    }
    pub fn alloc_id(&mut self) -> Id {
        for i in 0..1000
        {
            if !self.set.contains(&i)
            {
                self.set.insert(i);
                return Id(i);
            }
        }
        panic!("Can't allocate id");
    }
    pub fn remove_id(&mut self, id: i32) {
        self.set.remove(&id);
    }
}
