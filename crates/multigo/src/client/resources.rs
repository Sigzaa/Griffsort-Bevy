use bevy::prelude::*;
use std::net::SocketAddr;
use crate::prelude::*;


// #[derive(Default)]
// pub struct IterCount(pub i32);

// #[derive(Default)]
// pub struct IsRollback(pub bool);

// #[derive(Default)]
// pub struct RollTicker(pub i32);

// #[derive(Default)]
// pub struct ServerAddr(pub Option<SocketAddr>);

#[derive(Default)]
pub struct MyId(pub Option<f32>);


#[derive(Default)]
pub(crate) struct InternalShots(pub History<SnapShot>); // Collecting all client-made SnapShots in Buffer

#[derive(Default)]
pub(crate) struct ServerShots(pub History<SnapShot>); // =)

