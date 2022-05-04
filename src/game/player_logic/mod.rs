use bevy::prelude::*;
use crate::game::components::{
     filters::*, player_data::*, *};
mod physics;
pub mod client_controls;

pub struct Logic;
impl Plugin for Logic {
    fn build(&self, app: &mut App) {
        app
            //.add_system(client_controls::smooth_camera)
            //.add_system(client_controls::collect_inputs_sys)
            .add_system(physics::head_movement_system)
            //.add_system(physics::gravity)
            //.add_system(physics::movement)
            .add_system(respawn_system)
            .add_system(death_system)
            //.add_system(client_controls::client_shoot)
            ;
    }
}

fn respawn_system(
    mut commands: Commands,
    mut q_player: Query<(Entity, &mut Transform, &mut Killed, &mut Hp, &MaxHp, &Spawn), With<Core>>,
    time: Res<Time>,
) {
    for (player, mut transform, mut killed, mut hp, max_hp, spawn) in q_player.iter_mut() {
        killed.timer += time.delta_seconds();
        //println!("delta: {}, killed timer: {}", time.delta_seconds(), killed.timer);
        if killed.timer >= RESPAWNGAP {
            commands.entity(player).remove::<Killed>();
            transform.translation = spawn.respawn_coords;
            hp.0 = max_hp.0;
        }
    }
}
fn death_system(
    mut commands: Commands,
    mut q_player: Query<(Entity, &mut Transform, &Hp), (With<Hp>, Without<Killed>)>,
) {
    for (entity, mut transform, hp) in q_player.iter_mut() {
        if hp.0 <= 0 {
            commands.entity(entity).insert(Killed { timer: 0. });
            transform.translation = Vec3::new(0., -100., 0.);
            //commands.entity(entity).despawn_recursive();
        }
    }
}
