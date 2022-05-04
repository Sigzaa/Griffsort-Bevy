/*

Custom data struct for handling connected clients.

push -> i8 // returns index

remove -> void // removes

get (index) -> bool

should_refactor -> bool

impl index

*/

use std::net::SocketAddr;

use bevy::prelude::*;

pub struct AList {
    arr: [Node; 15],
}

#[derive(Debug, Clone, Copy)]
enum Node {
    Addr(SocketAddr),
    Empty,
}

impl Default for AList {
    fn default() -> Self {
        Self {
            arr: [Node::Empty; 15],
        } // Im shure that is not the best way.
    }
}

impl AList {
    pub fn len(&self) -> usize {
        self.arr.len()
    }
    pub fn insert(&mut self, addr: SocketAddr) -> Option<usize> {
        match self.find_empty() {
            Some(id) => {
                self.arr[id] = Node::Addr(addr);
                return Some(id);
            }
            None => None,
        }
    }
    #[allow(dead_code)]
    pub fn send_all(&self, _msg_type: &str, _msg: &str){
        
    }

    fn find_empty(&self) -> Option<usize> {
        for (i, x) in self.arr.iter().enumerate() {
            if matches!(x, Node::Empty) {
                return Some(i);
            }
        }
        None
    }
    fn find_not_empty(&self) -> Option<usize> {
        for (i, x) in self.arr.iter().enumerate() {
            if matches!(x, SocketAddr) {
                return Some(i);
            }
        }
        None
    }

    pub fn remove(&mut self, addr: SocketAddr) {
        match self.find(addr){
            Some(id) => self.arr[id] = Node::Empty,
            None => { error!("Can not remove {}: addres not found", addr)},
        }

    }
    fn find(&self, addr: SocketAddr) -> Option<usize>{
        for (i, x) in self.arr.iter().enumerate() {
            if matches!(x, addr) {
                return Some(i);
            }
        }
        None
    }
    pub fn print(&self) {
        println!("Connected List: ");
        for (i, x) in self.arr.iter().enumerate() {
            println!("  id {}: {:?}", i, x);
        }
    }
    pub fn is_empty(&self, index: usize) -> bool{
        matches!(self.arr[index], Node::Empty)
    }
    pub fn get_addr(&self, index: usize) -> Option<SocketAddr>{
        if let Node::Addr(address) = self.arr[index] {
            return Some(address);
        } else {
            None 
        }
    }
}
