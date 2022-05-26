use bevy::prelude::*;
use std::net::SocketAddr;
use crate::prelude::*;

#[derive(Default)]
pub struct TPS(pub Timer);

#[derive(Default)]
pub struct IterCount(pub i32);

#[derive(Default)]
pub struct IsRollback(pub bool);

#[derive(Default)]
pub struct RollTicker(pub i32);

#[derive(Default)]
pub struct ServerAddr(pub Vec<SocketAddr>); // TODO: Replace Vec by something better.

#[derive(Default)]
pub struct MyId(pub i32);


#[derive(Default)]
pub(crate) struct InternalShots(pub GoHistory<SnapShot>); // Collecting all client-made SnapShots in Buffer

#[derive(Default)]
pub(crate) struct ServerShots(pub GoHistory<SnapShot>); // =)

