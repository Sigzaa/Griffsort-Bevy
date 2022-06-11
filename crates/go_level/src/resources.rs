use bevy::prelude::*;

#[derive(Default)]
pub(crate) struct Loaded{
    path: &'static str,
    scene: Handle<Scene>
}