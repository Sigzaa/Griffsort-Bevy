use bevy::prelude::*;
use std::net::SocketAddr;
use crate::prelude::*;

#[derive(Default)]
pub struct MyId(pub Option<f32>);


#[derive(Default)]
pub(crate) struct InternalShots(pub History<SnapShot>); // Collecting all client-made SnapShots in Buffer

#[derive(Default)]
pub(crate) struct ServerShots(pub History<SnapShot>); // 
