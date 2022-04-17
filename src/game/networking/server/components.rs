use crate::game::components::player_data::*;
use priority_queue::PriorityQueue;
use bevy::prelude::*;

#[derive(Default)]
pub struct ConnectedList(pub super::AList);



#[derive(Default)]
pub struct TickCounter(pub i32);